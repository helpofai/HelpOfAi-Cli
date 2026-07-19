# Brain Cache & Incremental Update Strategy

Implements Engineering Law 8 (Cache Aggressively) and Law 14 (Knowledge Persistence).

## Cache Keying

Every brain query result is cached under a key derived from:
- Brain ID (AIOS-BRAIN-NNNNNN)
- Query parameters (sorted alphabetically)
- Current `graph_version`

Formula: `key = sha256(brain_id + "|" + sorted_query + "|" + graph_version)`

## Incremental Updates

When `file_indexing` (AIOS-CONTRACT-000041) runs:

1. **File-level** (fast): only re-index files whose `content_hash` changed.
2. **Symbol-level** (medium): re-parse changed files for symbols.
3. **Dependency-level** (slow): full dependency graph rebuild on package manifest changes.
4. **Feature-level** (on-demand): only when feature specs change.
5. **Decision/Risk/Knowledge**: appended only; never rebuilt unless explicitly requested.

## Cache Invalidation Rules

| Trigger | Invalidates |
|---------|-------------|
| File hash change | project_brain for that file, dependency_brain for its imports |
| Feature spec change | feature_brain for that feature |
| Dependency manifest change | dependency_brain fully |
| ADR recorded | decision_brain (append, no invalidation) |
| Risk mitigated | risk_brain for that risk |
| Knowledge learned | knowledge_brain (append) |
| Execution completes | execution_brain, history_brain |
| Operator requests re-index | all brains fully |

## Persistence

- Brain graphs are stored via memory module (AIOS-CONTRACT-000060) namespace `brain_graph`
- Cache entries use namespace `brain_cache`
- LRU eviction when `max_cache_mb` exceeded (declared in brain/module.json as 256 MB)