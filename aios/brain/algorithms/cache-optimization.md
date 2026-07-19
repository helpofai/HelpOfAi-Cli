# Cache Invalidation — Optimization Algorithm

## Purpose
Minimize unnecessary cache invalidations when only metadata (not content) changes.

## Algorithm
```
1. File hash changes → check if only whitespace or comments changed
2. If yes → DO NOT invalidate dependent caches
3. If no (actual logic change) → invalidate dependent caches
4. For dependency files (package.json, Cargo.toml):
   - Hash change = ALWAYS invalidate (dep changes affect everything)
5. For test files:
   - Hash change = invalidate test cache only, NOT production graph
```

## Cache Layers
```
L1: In-memory (hot) — 50ms TTL, 100MB max
L2: On-disk (warm) — 1hr TTL, 256MB max
L3: Cold storage — 24hr TTL, 512MB max
```

## Eviction Policy
- L1: LRU, evict oldest accessed first
- L2: LFU, evict least frequently used first
- L3: FIFO, evict oldest written first