# Performance Engine — Bottleneck Detection

## Detection Rules
```
Rule 1 — N+1 Query:
  Pattern: SELECT in loop
  Detection: >10 similar queries in single request
  Fix: Batch query or eager load

Rule 2 — Memory Leak:
  Pattern: Growing memory without GC
  Detection: Heap grows >10% per request
  Fix: Check for retained references, unclosed resources

Rule 3 — Slow Query:
  Pattern: Single query >500ms
  Detection: Query execution time exceeds threshold
  Fix: Add index, optimize JOIN, cache result

Rule 4 — Blocking I/O:
  Pattern: Synchronous file/network calls in hot path
  Detection: >100ms blocking calls in request path
  Fix: Make async, move to background job
```

## Scoring
```
score = 100 - (n_plus_1 * 15) - (memory_leak * 20) - (slow_query * 10) - (blocking_io * 10)
```