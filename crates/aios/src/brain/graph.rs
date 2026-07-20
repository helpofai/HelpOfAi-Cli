use anyhow::Result;
use rusqlite::{params, Connection};
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

        let file_id = uuid::Uuid::new_v4().to_string();

        tx.execute(
            "INSERT INTO code_files (file_id, relative_path, language, blake3_hash, line_count)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(relative_path) DO UPDATE SET
                blake3_hash = excluded.blake3_hash,
                line_count = excluded.line_count,
                last_indexed_at = CURRENT_TIMESTAMP",
            params![file_id, relative_path, language, blake3_hash, line_count as i64],
        )?;

        tx.execute(
            "DELETE FROM code_symbols WHERE file_id = (SELECT file_id FROM code_files WHERE relative_path = ?1)",
            params![relative_path],
        )?;

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
