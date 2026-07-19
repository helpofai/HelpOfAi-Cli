# Decision Journal — Export Configuration

## JSON Export Schema
```json
{
  "metadata": {"exported_at": "ISO8601", "count": "int", "filter": "string"},
  "entries": [
    {"id": "ADR-NNNNNN", "title": "string", "status": "enum", "tags": ["string"], "date": "ISO8601", "context": "string", "alternatives": "string", "decision": "string", "consequences": "string"}
  ]
}
```

## Markdown Export
One file per ADR or all-in-one with `---` separators.
Uses template from aiots/enterprise/decision-journal/exports/markdown-template.md.

## Command Reference
```
hoa decision export --all                        → all ADRs, Markdown
hoa decision export --id ADR-000003              → single ADR
hoa decision export --tag security               → filtered by tag
hoa decision export --status accepted             → accepted only
hoa decision export --since 2026-07-01            → date range
hoa decision export --format json                → JSON format
```