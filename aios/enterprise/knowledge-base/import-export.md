# Knowledge Base — Import/Export Pipeline

## Import Sources
- **Manual**: `hoa knowledge add` (interactive CLI)
- **Git history**: extract commit messages as knowledge entries
- **PR reviews**: extract recurring review comments
- **Decisions**: sync from Decision Journal ADRs
- **Framework packs**: pre-populated from `aios/frameworks/`

## Import Formats
| Format | Source | Command |
|--------|--------|---------|
| JSON | Bulk import | `hoa knowledge import entries.json` |
| Markdown | Single entry | `hoa knowledge add <file.md>` |
| CSV | Spreadsheet export | `hoa knowledge import kb.csv --format csv` |
| Git | Commit analysis | `hoa knowledge scan --git` |

## Export Formats
- JSON: full structured data
- Markdown: human-readable
- CSV: spreadsheet-compatible

## Sync Frequency
- Manual import: on demand
- Git scan: triggered by `--git` flag  
- Auto-import from PRs: via CI hook (optional)