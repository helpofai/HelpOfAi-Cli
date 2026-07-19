# Agent System — onDispatch Queuing

## Queue Priorities
```
CRITICAL (1): security review, production rollback
HIGH (2): feature build, bug fix, review
NORMAL (3): refactor, analysis, documentation
LOW (4): indexing, cleanup, optimization
```

## Queue Rules
- Only 3 HIGH+ agents may run concurrently
- CRITICAL preempts any running NORMAL/LOW task
- Tasks in the same priority are FIFO
- Queue depth > 10 = flag to operator

## Timeout Policy
```
Master agent: 300s max
Specialist agent: 180s max
Review agent: 120s max
Documentation agent: 60s max
```