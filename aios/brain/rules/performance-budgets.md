# Brain — Performance Budgets

## Indexing Budgets
| Operation | Max Time | Memory | Notes |
|-----------|----------|--------|-------|
| File hash scan (10k files) | 2s | 50MB | Uses git diff when available |
| Single file re-parse | 200ms | 10MB | Per file |
| Full re-index | 60s | 500MB | Only on explicit request |
| Graph save | 500ms | 100MB | To memory module |

## Query Budgets
| Operation | Max Time | Memory | Notes |
|-----------|----------|--------|-------|
| Symbol lookup | 50ms | 5MB | Uses in-memory index |
| Call graph (depth 2) | 200ms | 20MB | |
| Dependency impact | 100ms | 10MB | |
| Context pack assembly | 500ms | 50MB | |

## Cache Sizing
- In-memory index: 50MB (for 50k symbols)
- Graph serialized: 10MB (for 10k files)
- Knowledge brain: 5MB (per 1k entries)
- Total brain cache budget: 256MB (from module.json)