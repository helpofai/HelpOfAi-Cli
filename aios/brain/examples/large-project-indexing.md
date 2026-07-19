# Large Project Indexing Strategy

## Scenario
Monorepo with:
- 50,000 files across 20 packages
- 12 programming languages
- 200,000+ symbols
- 500+ features

## Strategy

### Layer 1 — Package-Level Index (fast)
```
Index only: package.json, Cargo.toml, pyproject.toml, go.mod
Symbols: none
Time: ~2s
Storage: 0.5MB
```
Use for: dependency resolution, build order, package queries

### Layer 2 — Public API Index (medium)
```
Index: public exports, interfaces, types, function signatures
Symbols: ~20,000 (public API only)
Time: ~30s
Storage: 5MB
```
Use for: feature impact analysis, dependency graph

### Layer 3 — Full Index (slow, on-demand)
```
Index: everything (all files, all symbols)
Symbols: ~200,000
Time: ~5min
Storage: 50MB
```
Use for: similarity matching, full text search, detailed analysis

## Auto-Scaling
- < 5,000 files: Layer 3 by default
- 5,000 - 50,000 files: Layer 2 default, Layer 3 on demand
- > 50,000 files: Layer 1 default, Layer 2 on demand, Layer 3 on explicit request