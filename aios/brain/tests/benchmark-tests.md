# Brain — Benchmark Tests

## Performance Benchmarks

### File Indexing Throughput
```
Input: 10,000 files (mixed TypeScript, Python, Rust)
Expected: full index < 30s, incremental (100 changed) < 500ms
```

### Query Latency
```
Input: TF-IDF query "validateToken" on 50k node graph
Expected: p50 < 50ms, p99 < 200ms
```

### Context Pack Assembly
```
Input: "build auth feature" on medium project (5k files, 20k symbols)
Expected: assembly < 500ms, pack < 4000 tokens
```

### Cycle Detection
```
Input: 10k node dependency graph
Expected: detection < 2s
```

### Similarity Matching
```
Input: 5k functions, threshold 0.7
Expected: index < 500ms, query < 200ms
```

## Concurrent Query Test
```
Input: 10 simultaneous queries
Expected: no timeout, all complete < 2x single-query latency
```

## Cache Performance
```
Input: same query twice
Expected: cache hit = 5ms, cache miss = 50ms
Cache hit rate target: > 80%
```