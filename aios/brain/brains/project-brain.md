# Project Brain — AIOS-BRAIN-000001

Maintains a versioned map of the entire project: files, directories, symbols,
imports, routes, and configuration. Updated incrementally — only changed files
are re-parsed (Law 8). The knowledge graph uses this as its base node set.

### Schema
- `entries[]`: path, kind (file|dir|symlink), size, content_hash
- `langs[]`: detected languages by file extension
- `frameworks[]`: detected frameworks from config files
- `last_indexed_at`: timestamp of last full index