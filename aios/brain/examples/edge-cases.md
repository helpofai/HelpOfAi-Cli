# Knowledge Graph — Edge Case Analysis

## Empty Project
```
Files: 0
Symbols: 0
→ Return empty graph. No errors. graph_version = 0.
```

## Single File
```
Files: 1
Symbols: 12 (from one TypeScript file)
→ Graph with 1 node + 12 symbol nodes. Edges: all "defines".
```

## Massive File
```
File: 50,000 lines, 500+ symbols
→ Index only top-level symbols (classes, interfaces, public functions)
→ Skip private/internal symbols to stay within budget
→ Flag file as "large — partial index"
```

## Circular Dependency
```
File A imports File B, File B imports File A
→ Cycle detected via Tarjan's SCC
→ Flag as CRITICAL
→ Suggest: extract shared dependency into separate file
```

## Binary Files
```
File: image.png, archive.zip
→ Skip indexing. Record existence only (type: "binary")
→ Do not attempt symbol extraction
```

## Encrypted/Compressed
```
File: secrets.env.enc, data.tar.gz
→ Skip entirely. No metadata recorded.
```

## Permission Denied
```
File: /etc/shadow (simulated)
→ Skip. Log warning. Continue indexing remaining files.
→ Do not crash or stall.
```