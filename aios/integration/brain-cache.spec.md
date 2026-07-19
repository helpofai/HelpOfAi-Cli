# Brain Cache — CLI Integration Spec

The CLI caches brain data between sessions for persistence and performance.

## Storage Location

`aios/.cache/brain/` — persists across CLI restarts.

## Cache Files

| File | Content | Invalidated By |
|------|---------|----------------|
| `project-map.json` | File index + hashes | File changes |
| `knowledge-graph.json` | Graph nodes + edges | File/symbol changes |
| `dependency-graph.json` | Module/symbol deps | Manifest changes |
| `decision-log.json` | ADR entries | New ADR (appended) |
| `timeline.json` | Engineering timeline | New events (appended) |

## Cache Flow

```
CLI start → read brain cache → check staleness
  → stale? run file_indexing → update cache → continue
  → fresh? use cached data → skip indexing
```

The cache is content-addressed: each entry is keyed by `sha256(inputs)`.
If inputs haven't changed, the cache hit returns immediately (Law 8).