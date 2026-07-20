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

    /// Perform a deep scan of the workspace, indexing code files into the SQLite Knowledge Graph.
    pub fn scan_and_index(&self, workspace_root: &Path) -> Result<usize> {
        let mut indexed = 0;

        for entry in walkdir::WalkDir::new(workspace_root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && is_code_file(path) {
                if let Ok(rel_path) = path.strip_prefix(workspace_root) {
                    let rel_str = rel_path.to_string_lossy().to_string();
                    if rel_str.contains(".git")
                        || rel_str.contains("target")
                        || rel_str.contains("node_modules")
                    {
                        continue;
                    }

                    if let Ok(content) = std::fs::read_to_string(path) {
                        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
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
                }
            }
        }

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

fn is_code_file(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(
        ext,
        "rs" | "ts" | "js" | "py" | "go" | "java" | "cpp" | "h" | "sql"
    )
}

fn detect_language(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => "rust",
        "ts" | "js" => "typescript",
        "py" => "python",
        "go" => "golang",
        "sql" => "sql",
        _ => "text",
    }
}
