# File Indexing — Incremental Update Algorithm

## Step 1: Detect Changes
```
git diff --name-only HEAD~1  → list changed files
Compare content_hash against stored hashes  → identify actual changes
```

## Step 2: Re-index Changed Files
```
for each changed file:
  detect language (file extension)
  parse symbols (functions, classes, interfaces, variables)
  extract imports/dependencies
  update node properties
  add/remove edges as needed
```

## Step 3: Propagate Changes
```
for each re-indexed file:
  update dependency edges (imports changed → re-check dependents)
  update feature mapping (file → feature edges)
  increment graph_version
```

## Step 4: Cache Update
```
save updated graph to memory module namespace "brain_graph"
increment graph_version
emit AIOS-EVENT-000040
```

## Performance
- 100 files changed: ~500ms
- Full re-index (10,000 files): ~30s
- Incremental (10 files): ~100ms