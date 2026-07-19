# Workflow — Performance Budgets

## Execution Budgets
| Workflow | Max Time | Parallel Phases |
|----------|----------|----------------|
| build-feature | 300s | 0 (sequential) |
| fix-bug | 180s | 0 |
| review-code | 60s | 1 (scan + analyze run in parallel) |
| refactor | 300s | 0 |
| optimize | 120s | 2 (analyze + suggest in parallel) |
| release | 600s | 1 (test + security in parallel) |
| rollback | 60s | 0 |

## Phase Budgets
| Phase Type | Max Time | Notes |
|------------|----------|-------|
| design | 60s | Architect generates plan |
| implement | 180s | Code generation |
| test | 60s | Test execution |
| review | 30s | Code review |
| deploy | 300s | External dependency |
| verify | 30s | Post-deploy verification |