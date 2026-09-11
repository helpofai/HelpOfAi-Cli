pub mod graph;
pub mod impact;
pub mod parser;
pub mod schema;

use anyhow::Result;
use graph::CodebaseKnowledgeGraph;
use impact::{ImpactEngine, ImpactReport};
use parser::AstParser;
use std::path::{Path, PathBuf};

pub struct ProjectBrain {
    aios_root: PathBuf,
    graph: CodebaseKnowledgeGraph,
}

impl ProjectBrain {
    pub fn open(aios_root: impl Into<PathBuf>) -> Result<Self> {
        let root = aios_root.into();
        let graph = CodebaseKnowledgeGraph::open(&root)?;
        Ok(Self {
            aios_root: root,
            graph,
        })
    }

    pub fn aios_root(&self) -> &Path {
        &self.aios_root
    }

    /// Return count of indexed files and symbols in the knowledge graph.
    pub fn stats(&self) -> Result<(usize, usize)> {
        let conn = self.graph.get_connection()?;
        let files: usize = conn
            .query_row("SELECT COUNT(*) FROM code_files", [], |r| r.get(0))
            .unwrap_or(0);
        let symbols: usize = conn
            .query_row("SELECT COUNT(*) FROM code_symbols", [], |r| r.get(0))
            .unwrap_or(0);
        Ok((files, symbols))
    }

    /// Perform a high-performance deep scan of the workspace, indexing code files into
    /// the SQLite Knowledge Graph with directory pruning and incremental hash caching.
    pub fn scan_and_index(&self, workspace_root: &Path) -> Result<usize> {
        let existing_hashes = self.graph.get_indexed_file_hashes().unwrap_or_default();
        let mut seen_paths = std::collections::HashSet::new();
        let mut indexed = 0;

        let walker = walkdir::WalkDir::new(workspace_root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|entry| !is_ignored_entry(entry));

        for entry in walker.filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !is_code_file(path) {
                continue;
            }

            // Skip oversized files (e.g. huge minified bundles, fixture dumps)
            if let Ok(meta) = entry.metadata() {
                if meta.len() > MAX_INDEXABLE_FILE_SIZE {
                    continue;
                }
            }

            let Ok(rel_path) = path.strip_prefix(workspace_root) else {
                continue;
            };
            let rel_str = rel_path.to_string_lossy().to_string();

            let Ok(content) = std::fs::read_to_string(path) else {
                continue;
            };

            let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
            seen_paths.insert(rel_str.clone());

            // Incremental check: if file hash is unchanged, skip AST parsing and DB write
            if existing_hashes.get(&rel_str) == Some(&hash) {
                indexed += 1;
                continue;
            }

            let lang = detect_language(path);
            if let Ok(symbols) = AstParser::parse_file(&content, &rel_str, lang) {
                let _ = self.graph.persist_file_symbols(
                    &rel_str,
                    lang,
                    &hash,
                    content.lines().count(),
                    &symbols,
                );
                indexed += 1;
            }
        }

        // Clean up any files that were deleted or moved
        let _ = self.graph.remove_stale_files(&seen_paths);

        Ok(indexed)
    }

    /// Perform multi-file impact analysis for a target class/function symbol.
    pub fn analyze_impact(&self, symbol_name: &str) -> Result<ImpactReport> {
        let engine = ImpactEngine::new(&self.graph);
        engine.analyze_target_symbol(symbol_name)
    }

    /// Format multi-file impact analysis as Markdown context.
    pub fn assemble_impact_markdown(&self, symbol_name: &str) -> String {
        let engine = ImpactEngine::new(&self.graph);
        if let Ok(report) = engine.analyze_target_symbol(symbol_name) {
            engine.format_impact_markdown(&report)
        } else {
            String::new()
        }
    }

    /// Query the Knowledge Graph for class/struct/fn symbols relevant to a prompt/task.
    pub fn assemble_precision_context(&self, query: &str, max_token_budget: usize) -> String {
        let Ok(conn) = self.graph.get_connection() else {
            return String::new();
        };

        let keywords: Vec<&str> = query
            .split(|c: char| c.is_whitespace() || c == '_' || c == '-' || c == '/' || c == '.')
            .filter(|w| w.len() > 2)
            .collect();

        if keywords.is_empty() {
            return String::new();
        }

        let mut output =
            String::from("\n\n## AIOS Project Brain — Exact Class & Code Symbol Context\n\n");
        let mut current_tokens = 20;

        for kw in keywords.iter().take(4) {
            let pattern = format!("%{kw}%");
            let mut stmt = match conn.prepare(
                "SELECT s.short_name, s.symbol_kind, s.signature, f.relative_path, s.start_line, s.docstring
                 FROM code_symbols s
                 JOIN code_files f ON s.file_id = f.file_id
                 WHERE s.short_name LIKE ?1 OR s.qualified_name LIKE ?1
                 LIMIT 4",
            ) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let rows = stmt.query_map(rusqlite::params![pattern], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            });

            if let Ok(rows) = rows {
                for r in rows.flatten() {
                    let (name, kind, sig, path, line, doc) = r;
                    let snippet = format!(
                        "* **{kind}** `{name}` ({path}:{line})\n  ```rust\n  {sig}\n  ```\n"
                    );
                    let snippet_tokens = snippet.len() / 4;

                    if current_tokens + snippet_tokens > max_token_budget {
                        break;
                    }

                    output.push_str(&snippet);
                    if let Some(d) = doc {
                        output.push_str(&format!("  *Doc:* {d}\n"));
                    }
                    output.push('\n');
                    current_tokens += snippet_tokens;
                }
            }
        }

        if current_tokens > 20 {
            output
        } else {
            String::new()
        }
    }
}

const MAX_INDEXABLE_FILE_SIZE: u64 = 512 * 1024; // 512 KB

fn is_ignored_entry(entry: &walkdir::DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();

    // Allow root directory
    if entry.depth() == 0 {
        return false;
    }

    // Prune known heavy dependency, build, cache, and artifact directories
    if entry.file_type().is_dir() {
        if name.starts_with('.') {
            return true;
        }
        return matches!(
            name.as_ref(),
            "node_modules"
                | "vendor"
                | "storage"
                | "target"
                | "dist"
                | "build"
                | "out"
                | "cache"
                | "coverage"
                | "venv"
                | "env"
                | "__pycache__"
        );
    }

    // Skip minified or bundle/lock files
    if name.ends_with(".min.js")
        || name.ends_with(".min.css")
        || name.ends_with(".map")
        || name.ends_with(".lock")
    {
        return true;
    }

    false
}

fn is_code_file(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(
        ext,
        "rs" | "ts"
            | "tsx"
            | "js"
            | "jsx"
            | "py"
            | "go"
            | "java"
            | "cpp"
            | "c"
            | "h"
            | "hpp"
            | "sql"
            | "php"
            | "cs"
            | "rb"
            | "swift"
            | "kt"
    )
}

fn detect_language(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" => "python",
        "go" => "golang",
        "sql" => "sql",
        "php" => "php",
        "java" => "java",
        "cpp" | "c" | "h" | "hpp" => "cpp",
        "cs" => "csharp",
        "rb" => "ruby",
        "swift" => "swift",
        "kt" => "kotlin",
        _ => "text",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_scan_and_index_prunes_ignored_directories_and_increments() {
        let tmp = tempdir().unwrap();
        let ws = tmp.path();

        // Create app code
        let app_dir = ws.join("app").join("Services");
        std::fs::create_dir_all(&app_dir).unwrap();
        std::fs::write(
            app_dir.join("Client.php"),
            "<?php\nclass Client {\n    public function run() {}\n}\n",
        )
        .unwrap();

        // Create vendor code that must be ignored
        let vendor_dir = ws.join("vendor").join("somepkg");
        std::fs::create_dir_all(&vendor_dir).unwrap();
        std::fs::write(
            vendor_dir.join("VendorClass.php"),
            "<?php\nclass VendorClass {}\n",
        )
        .unwrap();

        // Create storage directory that must be ignored
        let storage_dir = ws.join("storage").join("framework");
        std::fs::create_dir_all(&storage_dir).unwrap();
        std::fs::write(
            storage_dir.join("cached.php"),
            "<?php\nclass CachedView {}\n",
        )
        .unwrap();

        // Initialize Brain
        let aios_root = ws.join(".aios");
        let brain = ProjectBrain::open(&aios_root).unwrap();

        // First scan
        let indexed = brain.scan_and_index(ws).unwrap();
        assert_eq!(
            indexed, 1,
            "Only Client.php should be indexed; vendor and storage must be pruned"
        );

        let (files, symbols) = brain.stats().unwrap();
        assert_eq!(files, 1);
        assert_eq!(symbols, 2); // 1 class + 1 method

        // Second scan (incremental)
        let indexed_again = brain.scan_and_index(ws).unwrap();
        assert_eq!(
            indexed_again, 1,
            "Incremental scan should see 1 cached file"
        );

        let (files2, symbols2) = brain.stats().unwrap();
        assert_eq!(files2, 1);
        assert_eq!(symbols2, 2);
    }
}
