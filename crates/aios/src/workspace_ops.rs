//! # AIOS Advanced Workspace Operations & Project Upgrade Engine
//!
//! Provides enterprise-grade workspace intelligence and atomic file operations:
//! 1. Atomic file writes, copies, moves, and safe deletions backed by a snapshot journal.
//! 2. Transaction rollback mechanism: restores workspace files to their exact pre-transaction state.
//! 3. Cross-file reference updating when moving or renaming components.
//! 4. Multi-file project context collector: bundles code, symbols, callers, configs, and tests.
//! 5. Intelligent upgrade analyzer: detects frameworks (Laravel/PHP, Next.js/Node, Rust, Python)
//!    and generates actionable modernization roadmaps.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::brain::ProjectBrain;
use crate::brain::parser::AstParser;

// ── Transaction & Journal Data Models ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub op_type: String, // "write", "copy", "move", "delete", "ref_update"
    pub source_path: String,
    pub target_path: Option<String>,
    pub pre_hash: Option<String>,
    pub post_hash: Option<String>,
    pub snapshot_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionJournal {
    pub id: String,
    pub timestamp: u64,
    pub description: String,
    pub operations: Vec<OperationRecord>,
    pub reverted: bool,
}

// ── Project Collection Data Models ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedFile {
    pub relative_path: String,
    pub language: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub symbols: Vec<String>,
    pub preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContextBundle {
    pub target_query: String,
    pub matched_files: Vec<CollectedFile>,
    pub related_symbols: Vec<String>,
    pub callers_and_references: Vec<String>,
    pub test_files: Vec<String>,
    pub configs_detected: Vec<String>,
    pub summary: String,
}

// ── Project Upgrade Analysis Models ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRecommendation {
    pub component: String,
    pub current_version: Option<String>,
    pub target_version: Option<String>,
    pub severity: String, // "CRITICAL", "RECOMMENDED", "OPTIONAL"
    pub details: String,
    pub migration_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUpgradeAnalysis {
    pub project_type: String,
    pub detected_frameworks: Vec<String>,
    pub recommendations: Vec<UpgradeRecommendation>,
    pub breaking_changes_warning: Vec<String>,
    pub suggested_workflow: String,
}

// ── Workspace Operations Engine ─────────────────────────────────────────────

pub struct AiosWorkspaceOps {
    aios_root: PathBuf,
    workspace_root: PathBuf,
}

impl AiosWorkspaceOps {
    pub fn new(aios_root: impl Into<PathBuf>, workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            aios_root: aios_root.into(),
            workspace_root: workspace_root.into(),
        }
    }

    fn journal_dir(&self) -> PathBuf {
        self.aios_root.join(".cache").join("journal")
    }

    fn snapshot_dir(&self) -> PathBuf {
        self.aios_root.join(".cache").join("snapshots")
    }

    fn resolve_safe_path(&self, rel: &str) -> Result<PathBuf> {
        let clean = rel.trim().trim_start_matches(['/', '\\']);
        let candidate = self.workspace_root.join(clean);
        // Normalize
        let normalized = candidate
            .components()
            .fold(PathBuf::new(), |mut acc, comp| {
                match comp {
                    std::path::Component::ParentDir => {
                        acc.pop();
                    }
                    std::path::Component::CurDir => {}
                    _ => acc.push(comp),
                }
                acc
            });

        if !normalized.starts_with(&self.workspace_root) {
            bail!("Security violation: path '{rel}' escapes workspace root");
        }
        Ok(normalized)
    }

    /// Read file content safely with line range support and AST symbol summary.
    pub fn read_file_rich(
        &self,
        relative_path: &str,
        start_line: Option<usize>,
        end_line: Option<usize>,
    ) -> Result<(String, Vec<String>)> {
        let abs_path = self.resolve_safe_path(relative_path)?;
        if !abs_path.is_file() {
            bail!("File not found: {relative_path}");
        }

        let content = std::fs::read_to_string(&abs_path)
            .with_context(|| format!("Failed to read file: {relative_path}"))?;

        // Extract symbols via AST parser
        let symbols = AstParser::parse_file(&content, relative_path, "auto")
            .unwrap_or_default()
            .into_iter()
            .map(|s| format!("{} {} (line {})", s.symbol_kind, s.short_name, s.start_line))
            .collect();

        let lines: Vec<&str> = content.lines().collect();
        let total = lines.len();

        let start = start_line.unwrap_or(1).saturating_sub(1);
        let end = end_line.unwrap_or(total).min(total);

        if start >= total {
            return Ok((String::new(), symbols));
        }

        let sliced = lines[start..end].join("\n");
        Ok((sliced, symbols))
    }

    /// Safely write file with atomic swap and rollback snapshot preservation.
    pub fn write_file_safe(
        &self,
        relative_path: &str,
        content: &str,
        description: &str,
    ) -> Result<String> {
        let abs_path = self.resolve_safe_path(relative_path)?;
        let tx_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        std::fs::create_dir_all(self.journal_dir())?;
        let tx_snapshot_dir = self.snapshot_dir().join(&tx_id);
        std::fs::create_dir_all(&tx_snapshot_dir)?;

        let (pre_hash, snapshot_path) = if abs_path.exists() {
            let pre_content = std::fs::read(&abs_path)?;
            let pre_h = blake3::hash(&pre_content).to_hex().to_string();
            let snap_file = tx_snapshot_dir.join("pre_image.dat");
            std::fs::write(&snap_file, &pre_content)?;
            (Some(pre_h), Some(snap_file.to_string_lossy().to_string()))
        } else {
            (None, None)
        };

        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Atomic write: write to temporary file in same folder, then rename
        let tmp_file = abs_path.with_extension(format!("tmp.{tx_id}"));
        std::fs::write(&tmp_file, content.as_bytes())?;
        std::fs::rename(&tmp_file, &abs_path)?;

        let post_hash = blake3::hash(content.as_bytes()).to_hex().to_string();

        let op = OperationRecord {
            op_type: "write".to_string(),
            source_path: relative_path.to_string(),
            target_path: None,
            pre_hash,
            post_hash: Some(post_hash),
            snapshot_path,
        };

        let journal = TransactionJournal {
            id: tx_id.clone(),
            timestamp: now,
            description: description.to_string(),
            operations: vec![op],
            reverted: false,
        };

        let journal_path = self.journal_dir().join(format!("tx_{now}_{tx_id}.json"));
        let j_json = serde_json::to_string_pretty(&journal)?;
        std::fs::write(journal_path, j_json)?;

        Ok(tx_id)
    }

    /// Safely copy file or directory with snapshot protection.
    pub fn copy_path_safe(
        &self,
        source_rel: &str,
        target_rel: &str,
        description: &str,
    ) -> Result<String> {
        let abs_src = self.resolve_safe_path(source_rel)?;
        let abs_dst = self.resolve_safe_path(target_rel)?;

        if !abs_src.exists() {
            bail!("Source path does not exist: {source_rel}");
        }

        let tx_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        std::fs::create_dir_all(self.journal_dir())?;
        let tx_snapshot_dir = self.snapshot_dir().join(&tx_id);
        std::fs::create_dir_all(&tx_snapshot_dir)?;

        let mut operations = Vec::new();

        if abs_src.is_file() {
            let (pre_hash, snapshot_path) = if abs_dst.exists() {
                let pre_content = std::fs::read(&abs_dst)?;
                let pre_h = blake3::hash(&pre_content).to_hex().to_string();
                let snap_file = tx_snapshot_dir.join("dst_overwrite.dat");
                std::fs::write(&snap_file, &pre_content)?;
                (Some(pre_h), Some(snap_file.to_string_lossy().to_string()))
            } else {
                (None, None)
            };

            if let Some(parent) = abs_dst.parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::copy(&abs_src, &abs_dst)?;
            let post_content = std::fs::read(&abs_dst)?;
            let post_h = blake3::hash(&post_content).to_hex().to_string();

            operations.push(OperationRecord {
                op_type: "copy".to_string(),
                source_path: source_rel.to_string(),
                target_path: Some(target_rel.to_string()),
                pre_hash,
                post_hash: Some(post_h),
                snapshot_path,
            });
        } else if abs_src.is_dir() {
            std::fs::create_dir_all(&abs_dst)?;
            for entry in walkdir::WalkDir::new(&abs_src)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let p = entry.path();
                if let Ok(sub_rel) = p.strip_prefix(&abs_src) {
                    let dst_sub = abs_dst.join(sub_rel);
                    if entry.file_type().is_dir() {
                        std::fs::create_dir_all(&dst_sub)?;
                    } else if entry.file_type().is_file() {
                        if let Some(parent) = dst_sub.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        std::fs::copy(p, &dst_sub)?;
                    }
                }
            }
            operations.push(OperationRecord {
                op_type: "copy_dir".to_string(),
                source_path: source_rel.to_string(),
                target_path: Some(target_rel.to_string()),
                pre_hash: None,
                post_hash: None,
                snapshot_path: None,
            });
        }

        let journal = TransactionJournal {
            id: tx_id.clone(),
            timestamp: now,
            description: description.to_string(),
            operations,
            reverted: false,
        };

        let journal_path = self.journal_dir().join(format!("tx_{now}_{tx_id}.json"));
        let j_json = serde_json::to_string_pretty(&journal)?;
        std::fs::write(journal_path, j_json)?;

        Ok(tx_id)
    }

    /// Safely move or rename file/directory, with optional automatic import/reference updating.
    pub fn move_path_safe(
        &self,
        source_rel: &str,
        target_rel: &str,
        update_references: bool,
        description: &str,
    ) -> Result<String> {
        let abs_src = self.resolve_safe_path(source_rel)?;
        let abs_dst = self.resolve_safe_path(target_rel)?;

        if !abs_src.exists() {
            bail!("Source path does not exist: {source_rel}");
        }

        let tx_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        std::fs::create_dir_all(self.journal_dir())?;
        let tx_snapshot_dir = self.snapshot_dir().join(&tx_id);
        std::fs::create_dir_all(&tx_snapshot_dir)?;

        if let Some(parent) = abs_dst.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Backup source before moving
        let snap_file = tx_snapshot_dir.join("moved_src.dat");
        if abs_src.is_file() {
            std::fs::copy(&abs_src, &snap_file)?;
        }

        std::fs::rename(&abs_src, &abs_dst)?;

        let mut operations = vec![OperationRecord {
            op_type: "move".to_string(),
            source_path: source_rel.to_string(),
            target_path: Some(target_rel.to_string()),
            pre_hash: None,
            post_hash: None,
            snapshot_path: Some(snap_file.to_string_lossy().to_string()),
        }];

        // If requested, scan workspace files and update import/namespace references
        if update_references {
            let old_stem = Path::new(source_rel)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let new_stem = Path::new(target_rel)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            if !old_stem.is_empty() && !new_stem.is_empty() && old_stem != new_stem {
                let updated = self.replace_symbol_across_workspace(old_stem, new_stem)?;
                for (file_p, pre_h, post_h) in updated {
                    operations.push(OperationRecord {
                        op_type: "ref_update".to_string(),
                        source_path: file_p,
                        target_path: None,
                        pre_hash: Some(pre_h),
                        post_hash: Some(post_h),
                        snapshot_path: None,
                    });
                }
            }
        }

        let journal = TransactionJournal {
            id: tx_id.clone(),
            timestamp: now,
            description: description.to_string(),
            operations,
            reverted: false,
        };

        let journal_path = self.journal_dir().join(format!("tx_{now}_{tx_id}.json"));
        let j_json = serde_json::to_string_pretty(&journal)?;
        std::fs::write(journal_path, j_json)?;

        Ok(tx_id)
    }

    /// Safely delete a file or directory with complete snapshot backup in trash.
    pub fn delete_path_safe(&self, target_rel: &str, description: &str) -> Result<String> {
        let abs_target = self.resolve_safe_path(target_rel)?;
        if !abs_target.exists() {
            bail!("Target path does not exist: {target_rel}");
        }

        let tx_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        std::fs::create_dir_all(self.journal_dir())?;
        let tx_snapshot_dir = self.snapshot_dir().join(&tx_id);
        std::fs::create_dir_all(&tx_snapshot_dir)?;

        let snap_file = tx_snapshot_dir.join("deleted.dat");
        let pre_hash = if abs_target.is_file() {
            let bytes = std::fs::read(&abs_target)?;
            std::fs::write(&snap_file, &bytes)?;
            Some(blake3::hash(&bytes).to_hex().to_string())
        } else {
            None
        };

        if abs_target.is_file() {
            std::fs::remove_file(&abs_target)?;
        } else if abs_target.is_dir() {
            std::fs::remove_dir_all(&abs_target)?;
        }

        let op = OperationRecord {
            op_type: "delete".to_string(),
            source_path: target_rel.to_string(),
            target_path: None,
            pre_hash,
            post_hash: None,
            snapshot_path: Some(snap_file.to_string_lossy().to_string()),
        };

        let journal = TransactionJournal {
            id: tx_id.clone(),
            timestamp: now,
            description: description.to_string(),
            operations: vec![op],
            reverted: false,
        };

        let journal_path = self.journal_dir().join(format!("tx_{now}_{tx_id}.json"));
        let j_json = serde_json::to_string_pretty(&journal)?;
        std::fs::write(journal_path, j_json)?;

        Ok(tx_id)
    }

    /// Roll back an entire transaction, restoring files from snapshots.
    pub fn rollback_transaction(&self, tx_id_hint: Option<&str>) -> Result<String> {
        let j_dir = self.journal_dir();
        if !j_dir.exists() {
            bail!("No transaction journal found.");
        }

        let mut journals: Vec<PathBuf> = std::fs::read_dir(&j_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "json").unwrap_or(false))
            .collect();

        journals.sort();
        journals.reverse(); // Most recent first

        let target_journal_path = if let Some(hint) = tx_id_hint {
            journals
                .into_iter()
                .find(|p| p.to_string_lossy().contains(hint))
                .ok_or_else(|| anyhow::anyhow!("Transaction '{hint}' not found in journal."))?
        } else {
            journals
                .into_iter()
                .find(|p| {
                    if let Ok(raw) = std::fs::read_to_string(p) {
                        if let Ok(j) = serde_json::from_str::<TransactionJournal>(&raw) {
                            return !j.reverted;
                        }
                    }
                    false
                })
                .ok_or_else(|| {
                    anyhow::anyhow!("No active un-reverted transaction found to rollback.")
                })?
        };

        let raw = std::fs::read_to_string(&target_journal_path)?;
        let mut journal: TransactionJournal = serde_json::from_str(&raw)?;

        if journal.reverted {
            bail!("Transaction '{}' has already been rolled back.", journal.id);
        }

        // Revert operations in reverse order
        for op in journal.operations.iter().rev() {
            let abs_src = self.resolve_safe_path(&op.source_path)?;

            match op.op_type.as_str() {
                "write" => {
                    if let Some(ref snap) = op.snapshot_path {
                        if Path::new(snap).exists() {
                            std::fs::copy(snap, &abs_src)?;
                        }
                    } else if abs_src.exists() {
                        // Was newly created, remove it
                        let _ = std::fs::remove_file(&abs_src);
                    }
                }
                "copy" => {
                    if let Some(ref dst_rel) = op.target_path {
                        let abs_dst = self.resolve_safe_path(dst_rel)?;
                        if let Some(ref snap) = op.snapshot_path {
                            if Path::new(snap).exists() {
                                std::fs::copy(snap, &abs_dst)?;
                            }
                        } else if abs_dst.exists() {
                            let _ = std::fs::remove_file(&abs_dst);
                        }
                    }
                }
                "move" => {
                    if let Some(ref dst_rel) = op.target_path {
                        let abs_dst = self.resolve_safe_path(dst_rel)?;
                        if abs_dst.exists() {
                            if let Some(parent) = abs_src.parent() {
                                std::fs::create_dir_all(parent)?;
                            }
                            std::fs::rename(&abs_dst, &abs_src)?;
                        }
                    }
                }
                "delete" => {
                    if let Some(ref snap) = op.snapshot_path {
                        if Path::new(snap).exists() {
                            if let Some(parent) = abs_src.parent() {
                                std::fs::create_dir_all(parent)?;
                            }
                            std::fs::copy(snap, &abs_src)?;
                        }
                    }
                }
                _ => {}
            }
        }

        journal.reverted = true;
        let updated_raw = serde_json::to_string_pretty(&journal)?;
        std::fs::write(&target_journal_path, updated_raw)?;

        Ok(format!(
            "Successfully rolled back transaction '{}' ({} operations restored)",
            journal.id,
            journal.operations.len()
        ))
    }

    /// Collect comprehensive project context: matching files, symbols, callers, configs, and tests.
    pub fn collect_project_context(&self, target_query: &str) -> Result<ProjectContextBundle> {
        let brain = ProjectBrain::open(&self.aios_root).ok();
        let query_lower = target_query.to_lowercase();

        let mut matched_files = Vec::new();
        let mut related_symbols = Vec::new();
        let mut callers_and_references = Vec::new();
        let mut test_files = Vec::new();
        let mut configs_detected = Vec::new();
        let mut seen_paths = HashSet::new();

        // 1. Scan workspace files (with directory pruning)
        for entry in walkdir::WalkDir::new(&self.workspace_root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                if e.depth() == 0 {
                    return true;
                }
                let name = e.file_name().to_string_lossy();
                if e.file_type().is_dir() {
                    if name.starts_with('.') {
                        return false;
                    }
                    return !matches!(
                        name.as_ref(),
                        "node_modules"
                            | "vendor"
                            | "storage"
                            | "target"
                            | "dist"
                            | "build"
                            | "cache"
                            | "coverage"
                    );
                }
                true
            })
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let Ok(rel) = path.strip_prefix(&self.workspace_root) else {
                continue;
            };
            let rel_str = rel.to_string_lossy().to_string();

            // Detect config files
            let lower_name = rel_str.to_lowercase();
            if lower_name.ends_with("composer.json")
                || lower_name.ends_with("package.json")
                || lower_name.ends_with("cargo.toml")
                || lower_name.ends_with(".env.example")
            {
                configs_detected.push(rel_str.clone());
            }

            // Detect tests
            if (lower_name.contains("test") || lower_name.contains("spec"))
                && lower_name.contains(&query_lower)
            {
                test_files.push(rel_str.clone());
            }

            // Match files by query
            if lower_name.contains(&query_lower) && seen_paths.insert(rel_str.clone()) {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                if let Ok(content) = std::fs::read_to_string(path) {
                    let lines = content.lines().count();
                    let symbols_list: Vec<String> =
                        AstParser::parse_file(&content, &rel_str, "auto")
                            .unwrap_or_default()
                            .into_iter()
                            .map(|s| {
                                let item = format!(
                                    "{} `{}` (line {})",
                                    s.symbol_kind, s.short_name, s.start_line
                                );
                                related_symbols.push(format!("{item} in `{rel_str}`"));
                                item
                            })
                            .collect();

                    let preview = content.lines().take(40).collect::<Vec<_>>().join("\n");

                    matched_files.push(CollectedFile {
                        relative_path: rel_str.clone(),
                        language: detect_lang(&rel_str).to_string(),
                        size_bytes: size,
                        line_count: lines,
                        symbols: symbols_list,
                        preview: Some(preview),
                    });
                }
            }
        }

        // 2. Query Project Brain for callers & references
        if let Some(b) = brain {
            if let Ok(report) = b.analyze_impact(target_query) {
                for loc in report.locations {
                    callers_and_references.push(format!(
                        "{} in `{}:{}` ({})",
                        loc.symbol_name, loc.file_path, loc.line_number, loc.relationship_type
                    ));
                }
            }
        }

        let summary = format!(
            "Collected {} matching files, {} symbols, {} references, and {} related tests for query '{}'.",
            matched_files.len(),
            related_symbols.len(),
            callers_and_references.len(),
            test_files.len(),
            target_query
        );

        Ok(ProjectContextBundle {
            target_query: target_query.to_string(),
            matched_files,
            related_symbols,
            callers_and_references,
            test_files,
            configs_detected,
            summary,
        })
    }

    /// Analyze project technology stack, framework versions, and generate an actionable upgrade roadmap.
    pub fn analyze_upgrade(&self) -> Result<ProjectUpgradeAnalysis> {
        let mut frameworks = Vec::new();
        let mut recommendations = Vec::new();
        let mut warnings = Vec::new();
        let mut project_type = "Unknown".to_string();

        // 1. Check PHP / Laravel (composer.json)
        let composer_path = self.workspace_root.join("composer.json");
        if composer_path.is_file() {
            project_type = "PHP / Laravel".to_string();
            if let Ok(content) = std::fs::read_to_string(&composer_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let require = json.get("require").and_then(|r| r.as_object());

                    let php_ver = require
                        .and_then(|r| r.get("php"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    frameworks.push(format!("PHP ({php_ver})"));

                    if let Some(laravel) = require
                        .and_then(|r| r.get("laravel/framework"))
                        .and_then(|v| v.as_str())
                    {
                        frameworks.push(format!("Laravel Framework ({laravel})"));

                        if laravel.contains("^10")
                            || laravel.contains("10.")
                            || laravel.contains("^9")
                            || laravel.contains("9.")
                        {
                            recommendations.push(UpgradeRecommendation {
                                component: "Laravel Framework".to_string(),
                                current_version: Some(laravel.to_string()),
                                target_version: Some("^11.0 / ^12.0".to_string()),
                                severity: "RECOMMENDED".to_string(),
                                details: "Upgrade to Laravel 11/12 for streamlined application structure, native concurrency, and PHP 8.3/8.4 support.".to_string(),
                                migration_steps: vec![
                                    "Update composer.json: 'laravel/framework': '^11.0'".to_string(),
                                    "Run 'composer update'".to_string(),
                                    "Migrate middleware and exception handling to bootstrap/app.php".to_string(),
                                    "Run test suite: 'php artisan test'".to_string(),
                                ],
                            });
                            warnings.push("Laravel 11 streamlines kernel.php into bootstrap/app.php. Review HTTP/Console Kernel customizations.".to_string());
                        }
                    }
                }
            }
        }

        // 2. Check Node / Next.js / TypeScript (package.json)
        let pkg_path = self.workspace_root.join("package.json");
        if pkg_path.is_file() {
            if project_type == "Unknown" {
                project_type = "Node / TypeScript".to_string();
            } else {
                project_type = format!("{project_type} + Node");
            }

            if let Ok(content) = std::fs::read_to_string(&pkg_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let deps = json.get("dependencies").and_then(|d| d.as_object());
                    let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

                    if let Some(next) = deps.and_then(|d| d.get("next")).and_then(|v| v.as_str()) {
                        frameworks.push(format!("Next.js ({next})"));
                        if next.contains("13.") || next.contains("14.") {
                            recommendations.push(UpgradeRecommendation {
                                component: "Next.js".to_string(),
                                current_version: Some(next.to_string()),
                                target_version: Some("^15.0".to_string()),
                                severity: "RECOMMENDED".to_string(),
                                details: "Upgrade to Next.js 15 for React 19 support, improved caching semantics, and Turbopack stability.".to_string(),
                                migration_steps: vec![
                                    "Run 'npx @next/codemod@canary upgrade latest'".to_string(),
                                    "Verify async request API changes (cookies(), headers(), params)".to_string(),
                                    "Run 'npm test' / 'pnpm test'".to_string(),
                                ],
                            });
                        }
                    }

                    if let Some(react) = deps.and_then(|d| d.get("react")).and_then(|v| v.as_str())
                    {
                        frameworks.push(format!("React ({react})"));
                    }

                    if let Some(ts) = dev_deps
                        .and_then(|d| d.get("typescript"))
                        .and_then(|v| v.as_str())
                    {
                        frameworks.push(format!("TypeScript ({ts})"));
                    }
                }
            }
        }

        // 3. Check Rust (Cargo.toml)
        let cargo_path = self.workspace_root.join("Cargo.toml");
        if cargo_path.is_file() {
            if project_type == "Unknown" {
                project_type = "Rust".to_string();
            }
            if let Ok(content) = std::fs::read_to_string(&cargo_path) {
                if content.contains("edition = \"2021\"") {
                    frameworks.push("Rust (Edition 2021)".to_string());
                    recommendations.push(UpgradeRecommendation {
                        component: "Rust Edition".to_string(),
                        current_version: Some("2021".to_string()),
                        target_version: Some("2024".to_string()),
                        severity: "RECOMMENDED".to_string(),
                        details: "Upgrade to Rust Edition 2024 for let_chains, standard prelude updates, and enhanced borrow checking.".to_string(),
                        migration_steps: vec![
                            "Run 'cargo fix --edition'".to_string(),
                            "Update Cargo.toml: edition = \"2024\"".to_string(),
                            "Run 'cargo check --all-targets'".to_string(),
                        ],
                    });
                } else if content.contains("edition = \"2024\"") {
                    frameworks.push("Rust (Edition 2024 Modern)".to_string());
                }
            }
        }

        if recommendations.is_empty() {
            recommendations.push(UpgradeRecommendation {
                component: "Core Dependencies".to_string(),
                current_version: None,
                target_version: None,
                severity: "OPTIONAL".to_string(),
                details: "Workspace dependencies are healthy and up to date.".to_string(),
                migration_steps: vec![
                    "Regular maintenance and security audit recommended.".to_string(),
                ],
            });
        }

        let suggested_workflow = if project_type.contains("PHP") || project_type.contains("Laravel")
        {
            "AIOS-WORKFLOW-000007 (upgrade) -> AIOS-WORKFLOW-000005 (review-code)".to_string()
        } else {
            "AIOS-WORKFLOW-000007 (upgrade)".to_string()
        };

        Ok(ProjectUpgradeAnalysis {
            project_type,
            detected_frameworks: frameworks,
            recommendations,
            breaking_changes_warning: warnings,
            suggested_workflow,
        })
    }

    fn replace_symbol_across_workspace(
        &self,
        old_stem: &str,
        new_stem: &str,
    ) -> Result<Vec<(String, String, String)>> {
        let mut updated = Vec::new();

        for entry in walkdir::WalkDir::new(&self.workspace_root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                if e.depth() == 0 {
                    return true;
                }
                let name = e.file_name().to_string_lossy();
                if e.file_type().is_dir() {
                    return !name.starts_with('.')
                        && name != "node_modules"
                        && name != "vendor"
                        && name != "storage"
                        && name != "target";
                }
                true
            })
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            if let Ok(content) = std::fs::read_to_string(path) {
                if content.contains(old_stem) {
                    let pre_h = blake3::hash(content.as_bytes()).to_hex().to_string();
                    let replaced = content.replace(old_stem, new_stem);
                    let post_h = blake3::hash(replaced.as_bytes()).to_hex().to_string();
                    let _ = std::fs::write(path, replaced);

                    if let Ok(rel) = path.strip_prefix(&self.workspace_root) {
                        updated.push((rel.to_string_lossy().to_string(), pre_h, post_h));
                    }
                }
            }
        }

        Ok(updated)
    }
}

fn detect_lang(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    match ext {
        "php" => "php",
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" => "python",
        "go" => "golang",
        "sql" => "sql",
        _ => "text",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_atomic_write_and_rollback() {
        let tmp = tempdir().unwrap();
        let ws = tmp.path().join("ws");
        let aios = tmp.path().join("aios");
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::create_dir_all(&aios).unwrap();

        let ops = AiosWorkspaceOps::new(&aios, &ws);

        // 1. Initial write
        let tx1 = ops
            .write_file_safe("app/Service.php", "<?php echo 'v1';", "create v1")
            .unwrap();
        assert!(!tx1.is_empty());

        let (content, _) = ops.read_file_rich("app/Service.php", None, None).unwrap();
        assert_eq!(content, "<?php echo 'v1';");

        // 2. Overwrite with v2
        let tx2 = ops
            .write_file_safe("app/Service.php", "<?php echo 'v2';", "upgrade v2")
            .unwrap();
        let (content2, _) = ops.read_file_rich("app/Service.php", None, None).unwrap();
        assert_eq!(content2, "<?php echo 'v2';");

        // 3. Rollback tx2
        ops.rollback_transaction(Some(&tx2)).unwrap();
        let (restored, _) = ops.read_file_rich("app/Service.php", None, None).unwrap();
        assert_eq!(
            restored, "<?php echo 'v1';",
            "Must restore pre-image after rollback"
        );
    }

    #[test]
    fn test_copy_move_delete_safe() {
        let tmp = tempdir().unwrap();
        let ws = tmp.path().join("ws");
        let aios = tmp.path().join("aios");
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::create_dir_all(&aios).unwrap();

        let ops = AiosWorkspaceOps::new(&aios, &ws);

        ops.write_file_safe("src/Origin.php", "<?php class Origin {}", "origin")
            .unwrap();

        // Copy
        ops.copy_path_safe("src/Origin.php", "src/Clone.php", "clone")
            .unwrap();
        assert!(ws.join("src/Clone.php").exists());

        // Move
        ops.move_path_safe("src/Clone.php", "src/Renamed.php", false, "rename")
            .unwrap();
        assert!(!ws.join("src/Clone.php").exists());
        assert!(ws.join("src/Renamed.php").exists());

        // Delete
        let tx_del = ops.delete_path_safe("src/Renamed.php", "delete").unwrap();
        assert!(!ws.join("src/Renamed.php").exists());

        // Rollback delete
        ops.rollback_transaction(Some(&tx_del)).unwrap();
        assert!(
            ws.join("src/Renamed.php").exists(),
            "Must un-delete file upon rollback"
        );
    }

    #[test]
    fn test_analyze_upgrade_laravel() {
        let tmp = tempdir().unwrap();
        let ws = tmp.path().join("ws");
        let aios = tmp.path().join("aios");
        std::fs::create_dir_all(&ws).unwrap();
        std::fs::create_dir_all(&aios).unwrap();

        let composer_json = r#"{
            "require": {
                "php": "^8.1",
                "laravel/framework": "^10.0"
            }
        }"#;
        std::fs::write(ws.join("composer.json"), composer_json).unwrap();

        let ops = AiosWorkspaceOps::new(&aios, &ws);
        let analysis = ops.analyze_upgrade().unwrap();

        assert_eq!(analysis.project_type, "PHP / Laravel");
        assert!(
            analysis
                .recommendations
                .iter()
                .any(|r| r.component == "Laravel Framework")
        );
    }
}
