use anyhow::Result;
use rusqlite::{Connection, params};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::parser::ParsedSymbol;
use super::schema::SCHEMA_SQL;

pub struct CodebaseKnowledgeGraph {
    db_path: PathBuf,
}

impl CodebaseKnowledgeGraph {
    pub fn open(aios_root: &Path) -> Result<Self> {
        let cache_dir = aios_root.join(".cache").join("brain");
        std::fs::create_dir_all(&cache_dir)?;
        let db_path = cache_dir.join("codebase_graph.db");

        let conn = Connection::open(&db_path)?;
        conn.execute_batch(SCHEMA_SQL)?;

        Ok(Self { db_path })
    }

    pub fn get_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON;")?;
        Ok(conn)
    }

    /// Retrieve map of indexed relative paths to their blake3 hashes.
    pub fn get_indexed_file_hashes(&self) -> Result<HashMap<String, String>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT relative_path, blake3_hash FROM code_files")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut map = HashMap::new();
        for r in rows {
            let (path, hash) = r?;
            map.insert(path, hash);
        }
        Ok(map)
    }

    /// Remove indexed files that no longer exist in the workspace.
    pub fn remove_stale_files(&self, active_relative_paths: &HashSet<String>) -> Result<usize> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction()?;
        let existing: Vec<String> = {
            let mut stmt = tx.prepare("SELECT relative_path FROM code_files")?;
            stmt.query_map([], |r| r.get(0))?
                .filter_map(|r| r.ok())
                .collect()
        };
        let mut removed = 0;
        for path in existing {
            if !active_relative_paths.contains(&path) {
                tx.execute(
                    "DELETE FROM code_files WHERE relative_path = ?1",
                    params![path],
                )?;
                removed += 1;
            }
        }
        tx.commit()?;
        Ok(removed)
    }

    pub fn persist_file_symbols(
        &self,
        relative_path: &str,
        language: &str,
        blake3_hash: &str,
        line_count: usize,
        symbols: &[ParsedSymbol],
    ) -> Result<()> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction()?;

        // Retrieve existing file_id if present to preserve relationships and cascade cleanly
        let file_id: String = match tx.query_row(
            "SELECT file_id FROM code_files WHERE relative_path = ?1",
            params![relative_path],
            |r| r.get(0),
        ) {
            Ok(existing_id) => {
                tx.execute(
                    "UPDATE code_files SET language = ?1, blake3_hash = ?2, line_count = ?3, last_indexed_at = CURRENT_TIMESTAMP WHERE file_id = ?4",
                    params![language, blake3_hash, line_count as i64, existing_id],
                )?;
                tx.execute(
                    "DELETE FROM code_symbols WHERE file_id = ?1",
                    params![existing_id],
                )?;
                existing_id
            }
            Err(_) => {
                let new_id = uuid::Uuid::new_v4().to_string();
                tx.execute(
                    "INSERT INTO code_files (file_id, relative_path, language, blake3_hash, line_count)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        new_id,
                        relative_path,
                        language,
                        blake3_hash,
                        line_count as i64
                    ],
                )?;
                new_id
            }
        };

        for sym in symbols {
            let symbol_id = uuid::Uuid::new_v4().to_string();

            tx.execute(
                "INSERT INTO code_symbols (
                    symbol_id, file_id, qualified_name, short_name, symbol_kind,
                    signature, docstring, visibility, start_line, end_line
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    symbol_id,
                    file_id,
                    sym.qualified_name,
                    sym.short_name,
                    sym.symbol_kind,
                    sym.signature,
                    sym.docstring,
                    sym.visibility,
                    sym.start_line as i64,
                    sym.end_line as i64,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}
