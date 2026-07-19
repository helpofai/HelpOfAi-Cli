# Decision Journal — AIOS-ENTERPRISE-000040

Searchable, filterable view of all Architectural Decision Records.
Built on the Decision Brain (AIOS-BRAIN-000005).

## Features
- Full-text search across all ADRs
- Filter by status (accepted, proposed, deprecated, superseded)
- Filter by tags (security, architecture, performance, etc.)
- Export single ADR or batch as Markdown/PDF

## CLI Commands
```
hoa decision list              → list all ADRs
hoa decision search <query>    → full-text search
hoa decision show <id>         → view full ADR
hoa decision export <id>       → export as Markdown
```