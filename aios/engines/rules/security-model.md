# Engines — Security & Isolation Model

## Trust Model
Each engine runs with a declared trust level:
```
HIGH: analysis, review, performance (read-only, no filesystem writes)
MEDIUM: code, testing, bug (read + write to project files)
LOW: devops, documentation (read + write + external network)
```

## Write Scope
```
HIGH engines → no write access
MEDIUM engines → write to project files only (not kernel/runtime/constitution)
LOW engines → write to project files + external services (GitHub, Docker registry)
```

## Audit Trail
Every engine write is logged:
```
{engine_id, file_path, operation, timestamp, trace_back}
```

## Cross-Engine Rules
- Engines cannot impersonate each other
- Engine A cannot request Engine B to perform a write on its behalf
- All cross-engine communication goes through the brain event bus
- Unauthorized access results in BLOCKED + audit event