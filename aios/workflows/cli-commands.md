# Workflow — CLI Commands

## Execution
```
hoa build feature "<description>"        → run build-feature workflow
hoa fix bug "<description>"              → run fix-bug workflow
hoa review [path]                        → run review-code workflow
hoa refactor [path]                      → run refactor workflow
hoa upgrade [package]                    → run upgrade workflow
hoa optimize                             → run optimize workflow
hoa analyze [module]                     → run analyze workflow
hoa release [version]                    → run release workflow
```

## Lifecycle
```
hoa workflow list                        → list available workflows
hoa workflow info <workflow_id>          → phase breakdown
hoa workflow status <task_id>            → current execution status
hoa workflow cancel <task_id>            → cancel running workflow
```

## Rollback
```
hoa rollback                             → rollback last workflow
hoa rollback <workflow_id>               → rollback specific workflow
hoa rollback --list                      → list rollbackable workflows
hoa rollback --preview <wf_id>           → preview rollback changes
```

## Debug
```
hoa workflow debug --trace <task_id>     → trace execution path
hoa workflow debug --profile <task_id>   → phase timing profile
```