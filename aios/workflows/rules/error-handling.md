# Workflow — Error Handling

## Error Codes
| Code | Meaning | Action |
|------|---------|--------|
| WORKFLOW_GATE_FAILED | Gate condition not met | Retry or rollback |
| WORKFLOW_PHASE_TIMEOUT | Phase exceeded time budget | Skip phase or rollback |
| WORKFLOW_CAPABILITY_UNAVAILABLE | Required module not loaded | Suggest profile change |
| WORKFLOW_DEADLOCK | Circular phase dependency | Abort, report to operator |
| WORKFLOW_ATTACHMENT_FAILED | File write failed | Retry, then abort |
| WORKFLOW_ROLLBACK_FAILED | Rollback script failed | Manual intervention required |

## Recovery Strategies
```
GATE_FAILED → if retry_count < 3: retry phase, else rollback
PHASE_TIMEOUT → if phase is optional: skip, else rollback
CAPABILITY_UNAVAILABLE → suggest loading module via profile
ROLLBACK_FAILED → escalate to operator (cannot auto-recover)
```