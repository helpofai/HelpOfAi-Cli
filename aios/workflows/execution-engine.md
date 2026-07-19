# Workflow Engine — Execution Algorithm

## Phase Sequencing
```
1. Parse workflow definition (JSON) → ordered list of phases
2. For each phase:
   a. Check phase preconditions (all deps resolved?)
   b. Dispatch to capability (via kernel)
   c. Wait for result (synchronous within phase)
   d. Evaluate gate conditions
   e. If gate passes: phase complete → next phase
   f. If gate fails: log failure → rollback or retry decision
3. All phases complete → workflow complete
4. Any gate fail beyond retry → workflow failed → rollback
```

## Parallel Execution
```
phases declared with "parallel: true" run concurrently
parallel phases must have no dependency chain
max parallel branches: 3
```

## Gate Passing
```
gate type "quality": score >= 60 → pass, else fail
gate type "review": critical == 0 → pass, else fail
gate type "test": all tests passed → pass, else fail
gate type "safety": no blockers → pass, else fail
gate type "manual": operator must confirm → suspended until input
```

## Retry Policy
```
automatic_retry: 1 (retry once on transient failure)
max_retries: 3 (configurable per workflow)
rollback_on_final_failure: true
```