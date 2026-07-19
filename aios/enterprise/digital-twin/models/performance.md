# Digital Twin — Cache & Performance

## Prediction Cache
Simulation results are cached by plan hash:
```
cache_key = sha256(plan_id + knowledge_graph_version)
TTL = 3600 seconds (1 hour)
Invalidated when the knowledge graph is updated
```

## Performance Budget
| Operation | Max | Warning |
|-----------|-----|---------|
| Impact scoring | 500ms | >1s |
| Conflict detection | 2s | >5s per plan |
| Full simulation | 5s | >15s |
| Cache lookup | 50ms | >100ms |

## Large Project Handling
For projects with >10,000 files, the Digital Twin auto-throttles:
- File-level conflict checks only (skip symbol-level)
- Limit concurrent simulations to 3
- Use cached knowledge graph snapshot (max 5 min old)