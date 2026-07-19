# Brain Cache — Algorithm

## Cache Strategy
```
1. On brain.index() completion: serialize graph to JSON
2. Store in aios/.cache/brain/graph_{version}.json
3. On brain.query(): 
   - Check if aios/.cache/brain/version file exists
   - Compare version against graph_version
   - If match: load from cache (fast path)
   - If no match: re-index (slow path)
```

## Cache Invalidation
```
graph_version_changed → invalidate all caches
module_added → partial invalidation (only affected capabilities)
module_removed → partial invalidation
```

## File-Based Cache
```
aios/.cache/brain/
├── version              → current graph version
├── graph_{v}.json       → full graph at version v
├── query_cache.jsonl    → recent query results (LRU, 1000 entries)
└── index_stats.json     → indexing metadata (last_indexed_at, file_count)
```