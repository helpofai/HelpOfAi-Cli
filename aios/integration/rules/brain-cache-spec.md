# Brain Cache — Implementation Spec

## Core Functions
```rust
fn read_cached_graph(cache_dir: &Path) -> Option<KnowledgeGraph>
fn write_cached_graph(cache_dir: &Path, graph: &KnowledgeGraph) -> Result<(), CacheError>
fn get_cache_version(cache_dir: &Path) -> Option<u64>
fn invalidate_cache(cache_dir: &Path) -> Result<(), CacheError>
```

## File Structure
```
aios/.cache/brain/
├── version                → u64 stored as decimal text
├── graph_{version}.json   → serialized KnowledgeGraph
├── query_cache.jsonl      → newline-delimited JSON query cache
└── index_stats.json       → indexing metadata
```

## Query Cache (LRU)
```rust
struct QueryCache {
    max_entries: usize,       // default 1000
    entries: VecDeque<CacheEntry>,
}

struct CacheEntry {
    query_hash: String,       // sha256 of query string
    query: String,
    result: QueryResult,
    timestamp: u64,           // epoch ms
    hit_count: u64,
}
```

## Cache Invalidation
- `invalidate_all()`: delete all files in cache dir, reset version to 0
- `update_version(v)`: write new version to version file
- `trim_cache()`: remove entries with lowest hit_count (run during indexing)