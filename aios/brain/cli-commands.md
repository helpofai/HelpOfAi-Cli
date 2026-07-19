# Brain — Command Reference

## Indexing
```
hoa brain index                    → full re-index
hoa brain index --incremental      → changed files only
hoa brain status                   → check index freshness
```

## Queries
```
hoa brain query "callers of AuthService"     → symbol resolution
hoa brain query "what imports User"           → dependency impact
hoa brain query "features using auth"         → feature-to-file mapping
hoa brain query "files changed since v1.0"    → change tracking
```

## Cache
```
hoa brain cache clear              → clear brain cache
hoa brain cache status             → cache hit/miss stats
hoa brain cache size               → current cache usage
```

## Diagnostics
```
hoa brain stats                    → index size, symbol count, edge count
hoa brain stats --verbose          → per-language breakdown
hoa brain health                   → staleness, performance, error counts
```