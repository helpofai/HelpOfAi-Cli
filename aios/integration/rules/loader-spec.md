# Module Loader — Implementation Spec

## Core Function
```rust
fn load_modules(registry_path: &Path, profile: &Profile) -> Result<LoadSummary, LoaderError>
```

## Steps
1. Read registry files (modules.json, capabilities.json)
2. Build module dependency graph (DAG)
3. Topological sort by load_order
4. For each module in order:
   - Read module.json manifest
   - Validate against schema (SCHEMA-000011)
   - Parse capabilities list
   - Load contracts (contracts/*.json)
   - Register module + capabilities with kernel
5. Handle errors:
   - Missing manifest → skip, return error for that module
   - Invalid schema → skip, return error
   - Missing dep → skip, return error with dependency info
6. Return summary

## Edge Cases
- Empty registry (no modules.json) → return empty summary, no error
- Only one module loaded → no dependency issues
- All modules depend on kernel → kernel must be first
- Profile with invalid module IDs → skip unknown IDs, warn