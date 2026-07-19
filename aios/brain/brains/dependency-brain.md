# Dependency Brain — AIOS-BRAIN-000003

Three-level dependency tracking: module-level (AIOS modules), package-level
(npm/cargo/pip deps), and symbol-level (imports/calls). Cycle detection runs
on every update.

### Schema
- `nodes[]`: id, type (module|package|symbol)
- `edges[]`: from, to, type (depends|imports|calls|provides), version_constraint
- `cycles[]`: detected cycles with path
- `orphans[]`: modules/packages not depended-on by anything

### Integration
- Planner risk_analysis (AIOS-CONTRACT-000031) reads cycles for risk scoring.
- Module loader (AIOS-CONTRACT-000012) reads this for dependency resolution.