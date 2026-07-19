# Brain — Advanced CLI Commands

## Batch Operations
```
hoa brain index --batch-size 100       → index in batches of 100 files
hoa brain index --parallel 4           → use 4 parallel workers
hoa brain query "..." --format json    → structured output
hoa brain query "..." --format csv     → CSV output
hoa brain query "..." --limit 50       → limit results
```

## Watch Mode
```
hoa brain watch                        → watch filesystem for changes
hoa brain watch --debounce 2000        → debounce 2s before re-indexing
hoa brain watch --exclude "tests/**"   → exclude test files
```

## Export/Import
```
hoa brain export --format jsonl > brain-dump.jsonl
hoa brain export --format cytoscape > graph.cyjs
hoa brain import --merge brain-dump.jsonl
```

## Diagnostics (Verbose)
```
hoa brain stats --json                 → machine-readable stats
hoa brain stats --by-language          → per-language breakdown
hoa brain stats --by-package           → per-package breakdown
hoa brain health --full                → comprehensive health report
hoa brain health --recent-errors 20    → last 20 errors
```

## Debug
```
hoa brain debug --trace-query "AuthService"   → trace query execution plan
hoa brain debug --profile-indexing             → profile indexing step durations
hoa brain debug --show-graph --depth 2         → show subgraph for debugging
```