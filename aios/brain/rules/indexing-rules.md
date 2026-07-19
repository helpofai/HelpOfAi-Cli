# File Indexing — Rules and Conventions

## Which Files to Index
- Source code: `.ts`, `.js`, `.py`, `.rs`, `.go`, `.java`, `.kt`, `.swift`, `.php`
- Config: `.json`, `.yaml`, `.toml`, `.env`, `.ini`
- Markup: `.md`, `.html`, `.css`, `.scss`
- Database: `.sql`, `.prisma`, `.migration`
- **Skip**: `node_modules/`, `vendor/`, `target/`, `.git/`, `dist/`, `build/`, `.cache/`

## Symbol Extraction
| Language | Symbols Extracted |
|----------|------------------|
| TypeScript | class, interface, function, type, enum, const, import |
| Python | class, def, async def, import, from import |
| Rust | fn, struct, enum, trait, impl, mod, use |
| Go | func, type, struct, interface, import |

## Edge Detection
- `imports`: file A imports file B
- `calls`: function A calls function B
- `defines`: file defines symbol
- `implements`: file implements feature
- `contains`: directory contains file

## Staleness
- Graph version < file modification time → STALE
- Auto-refresh every 30 minutes while CLI is active
- Force refresh on `hoa brain index`