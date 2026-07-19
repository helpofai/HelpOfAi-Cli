# Decision Journal — Search Config

## Search Fields
- `title` — weighted 0.5
- `summary` — weighted 0.3
- `tags` — weighted 0.15
- `status` — exact filter only

## Filter Operators
```
hoa decision list                              → all ADRs
hoa decision list --status accepted            → accepted only
hoa decision list --tag security               → security tagged
hoa decision list --status proposed --tag auth  → combined filter
hoa decision search "JWT rotation"             → full-text
hoa decision search "database" --limit 5       → top 5 results
```

## Batch Export
```
hoa decision export --all --format markdown > all-adrs.md
hoa decision export --tag security --format json > security-adrs.json
```