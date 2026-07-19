# Brain — Health Check Reference

## Health Dimensions
| Dimension | Checks | Scoring |
|-----------|--------|---------|
| Index Freshness | graph_version vs latest file modification | 0=stale, 100=fresh |
| Query Performance | p50/p99 latency vs budgets | 0=slow, 100=fast |
| Cache Hit Rate | hit_rate vs target (80%) | 0=poor, 100=excellent |
| Error Rate | errors per 1000 operations | 0=poor, 100=perfect (0 errors) |
| Memory Usage | current_mb vs budget (256MB) | 0=over, 100=under |

## Health Score
```
health = (freshness * 0.25) + (performance * 0.25) + (cache * 0.20) + (errors * 0.20) + (memory * 0.10)
```

## CLI Output
```
hoa brain health
→ Overall: 87/100 🟢
  Freshness:  92/100  🟢 (indexed 2 min ago)
  Perf:       85/100  🟢 (avg 38ms, p99 156ms)
  Cache:      89/100  🟢 (89% hit rate)
  Errors:     95/100  🟢 (2 errors in last 1000 ops)
  Memory:     52/100  🟡 (124MB of 256MB)
```