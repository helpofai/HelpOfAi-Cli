# Brain — Data Migration Guide

## Scenario: Moving Brain Data Between Projects

### Export
```
hoa brain export --format jsonl > project-a-brain.jsonl
hoa brain export --format cytoscape > project-a-graph.cyjs
```

### Import
```
hoa brain import --merge project-a-brain.jsonl
hoa brain import --replace project-a-brain.jsonl   # destructive!
```

### Merge Strategy
When importing brain data from another project:
1. Match nodes by content_hash (not file path — paths differ between projects)
2. If hash matches → reuse, add path alias
3. If hash doesn't match → treat as new file, index independently
4. Feature references are NOT merged across projects (different feature IDs)

### Conflict Resolution
```
Conflict: same content_hash, different path
→ Create alias: new_path → content_hash
→ Do not duplicate the node

Conflict: same symbol name, different content
→ Treat as separate symbols (same name, different context)
→ Do not merge
```