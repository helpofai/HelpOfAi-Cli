# Execution Brain — AIOS-BRAIN-000007

Records every execution trace. The kernel's state machine (AIOS-CONTRACT-000013)
writes transitions here; this brain enables resumability and audit.

### Schema
- `phases[]`: phase name, status (pending|active|passed|failed|skipped)
- `gate`: the gate that passed or failed
- `artifacts[]`: files produced during this phase
- `rollbacks[]`: record of rollbacks (Engineering Law 4)

### Integration
- Kernel reads previous state on restart to resume paused requests
- Timeline brain reads this for engineering timeline visualization