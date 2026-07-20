pub const SCHEMA_SQL: &str = r#"
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA foreign_keys = ON;

-- 1. Tracked Workspace Files
CREATE TABLE IF NOT EXISTS code_files (
    file_id TEXT PRIMARY KEY,
    relative_path TEXT NOT NULL UNIQUE,
    language TEXT NOT NULL,
    blake3_hash TEXT NOT NULL,
    line_count INTEGER NOT NULL,
    last_indexed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. Primary Code Symbols (Classes, Structs, Traits, Interfaces, Enums, Functions)
CREATE TABLE IF NOT EXISTS code_symbols (
    symbol_id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    qualified_name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    symbol_kind TEXT NOT NULL,
    signature TEXT NOT NULL,
    docstring TEXT,
    visibility TEXT NOT NULL,
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    FOREIGN KEY (file_id) REFERENCES code_files(file_id) ON DELETE CASCADE
);

-- 3. Class/Struct Members (Methods, Fields, Enum Variants)
CREATE TABLE IF NOT EXISTS symbol_members (
    member_id TEXT PRIMARY KEY,
    parent_symbol_id TEXT NOT NULL,
    member_name TEXT NOT NULL,
    member_kind TEXT NOT NULL,
    signature TEXT NOT NULL,
    visibility TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    FOREIGN KEY (parent_symbol_id) REFERENCES code_symbols(symbol_id) ON DELETE CASCADE
);

-- 4. Graph Relationships (Callers, Callee, Implementation, Inheritance)
CREATE TABLE IF NOT EXISTS code_relationships (
    relationship_id TEXT PRIMARY KEY,
    source_symbol_id TEXT NOT NULL,
    target_symbol_id TEXT NOT NULL,
    relation_type TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    FOREIGN KEY (source_symbol_id) REFERENCES code_symbols(symbol_id) ON DELETE CASCADE,
    FOREIGN KEY (target_symbol_id) REFERENCES code_symbols(symbol_id) ON DELETE CASCADE
);

-- Indexes for Fast Graph Queries
CREATE INDEX IF NOT EXISTS idx_symbols_file ON code_symbols(file_id);
CREATE INDEX IF NOT EXISTS idx_symbols_short ON code_symbols(short_name);
CREATE INDEX IF NOT EXISTS idx_symbols_kind ON code_symbols(symbol_kind);
CREATE INDEX IF NOT EXISTS idx_members_parent ON symbol_members(parent_symbol_id);
CREATE INDEX IF NOT EXISTS idx_rel_source ON code_relationships(source_symbol_id);
CREATE INDEX IF NOT EXISTS idx_rel_target ON code_relationships(target_symbol_id);
"#;
