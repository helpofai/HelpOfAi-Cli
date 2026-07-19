# Integration Scenario: First Load

## Context
Operator runs AIOS for the first time on a new project.

## Steps
1. CLI detects `aios/` directory at workspace root
2. Integration module loads:
   - Scans `aios/registry/` for registry files
   - Validates all JSON files
   - Builds in-memory module index
3. Module loader runs:
   - Sorts modules by load_order
   - Loads kernel → runtime → planner → brain → engines → agents → workflows
   - Each module validates its manifest against SCHEMA-000011
   - Each module registers capabilities with the kernel
4. Brain initializes:
   - Brain cache check: no cache found (first run)
   - Brain status: BRAIN_NOT_INDEXED
5. System reports ready:
   - `hoa module list` → 28 modules loaded
   - `hoa capability list` → 34 capabilities available
   - `hoa brain status` → "not indexed, run `hoa brain index`"