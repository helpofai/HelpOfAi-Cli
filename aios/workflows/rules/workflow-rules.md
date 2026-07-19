# Workflow — Rules and Conventions

## Naming
- Workflow IDs: `AIOS-WORKFLOW-NNNNNN` (6 digits, zero-padded)
- Task IDs: `WF-YYYYMMDD-NNN` (date + sequence, resets daily)
- Phase IDs: lowercase_kebab (e.g., `build_image`, `run_tests`)

## Phase Conventions
- Phase `design` is always first (if present)
- Phase `deploy` always uses `manual` gate
- Phase `review` is always last (if present)
- All phases should have a gate (even if `auto`)
- Gates of type `auto` always pass by default

## Rollback Policy
- Read-only workflows (review, analyze): no rollback
- Write workflows (build, fix, refactor): full rollback
- Release workflows: blue-green (automatic)

## Dependency Ordering
```
kernel → planner → brain → engines → agents → workflows
workflows load at order 35 (after agents at 30)
```