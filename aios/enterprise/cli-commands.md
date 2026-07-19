# Enterprise Features — CLI Commands Reference

## Digital Twin
```
hoa simulate <plan_id>              → simulate plan impact
hoa conflict list                   → show conflicts
hoa conflict resolve <id>           → resolve conflict
```

## Capability Graph
```
hoa cap-graph                       → show full graph
hoa cap-graph --module <id>         → filter by module
hoa cap-graph --cap <id>            → trace capability route
hoa cap-graph --export dot          → export as GraphViz
```

## Engineering Timeline
```
hoa timeline                        → recent events
hoa timeline --since 2 weeks ago    → date range
hoa timeline --type rollback        → filter by type
hoa timeline --export audit.md      → export for compliance
```

## Decision Journal
```
hoa decision list                   → all ADRs
hoa decision search <query>         → full-text search
hoa decision show <id>              → view ADR
hoa decision export <id> --md       → export as Markdown
```

## Project Health
```
hoa health                          → dashboard
hoa health --refresh                → force re-scan
hoa health metric architecture      → metric detail
hoa health --export report.md       → export report
```

## Learning Engine
```
hoa learning proposals              → pending proposals
hoa learning approve <id>           → approve
hoa learning reject <id>            → reject
```

## Knowledge Base
```
hoa knowledge search <query>        → search
hoa knowledge add                   → add entry
hoa knowledge export <id>           → export
```

## Roadmap Generator
```
hoa roadmap --features F1,F2,F3     → generate roadmap
hoa roadmap --capacity 3            → 3 engineers
hoa roadmap --start 2026-08-01      → start date
```