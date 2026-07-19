# Brain — Error Handling Reference

## Error Codes

| Code | Message | Recovery |
|------|---------|----------|
| BRAIN_NOT_INDEXED | No knowledge graph exists | `hoa brain index` to build one |
| BRAIN_STALE | Graph older than latest file change | `hoa brain index --incremental` |
| BRAIN_PARSE_FAILED | File could not be parsed | Record as unparsed, continue |
| BRAIN_WRITE_FAILED | Memory persist failed | Keep in-memory, degrade to warning |
| BRAIN_TOO_LARGE | >50k files, consider layered index | Use Layer 1 or 2 indexing |
| BRAIN_TIMEOUT | Query exceeded performance budget | Retry with simpler query |
| BRAIN_CYCLE_DETECTED | Circular dependency found | Surface cycle path, advise refactor |
| BRAIN_CACHE_CORRUPT | Stored value failed re-validation | Drop entry, return miss, log event |

## Graceful Degradation
```
BRAIN_PARSE_FAILED → continue indexing (skip unparsed files)
BRAIN_WRITE_FAILED → keep in-memory graph, retry persist every 5 min
BRAIN_TIMEOUT → return partial results, flag truncation
BRAIN_CACHE_CORRUPT → drop entry, re-compute on next query
```

## Recovery Commands
```
hoa brain cache clear           → clear all caches
hoa brain index --force         → force full re-index
hoa brain repair                → attempt automatic repair
hoa brain health --recent-errors → view recent errors
```