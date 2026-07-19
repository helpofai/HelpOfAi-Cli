# Workflow Walkthrough: release

## Input
```
hoa release v1.2.0
```

## Phase 1: Test
```
Agent: qa
Input: all test suites
Output: 245/245 passed, coverage 87%
Gate: test (all passed) → PASS
Duration: 180s
```

## Phase 2: Security
```
Agent: security
Input: full project scan
Output: 0 critical CVEs, 2 medium (accepted exceptions)
Gate: safety (no blockers) → PASS
Duration: 25s
```

## Phase 3: Review
```
Agent: reviewer
Input: diff from last release
Output: 0 critical, 1 major
Gate: review (critical=0) → PASS
Duration: 15s
```

## Phase 4: Deploy
```
Gate: manual → AWAITING OPERATOR CONFIRMATION
Operator confirms → deploy to production
```

## Phase 5: Verify
```
Agent: qa
Input: smoke tests on production
Output: 12/12 smoke tests passed
Gate: test (all passed) → PASS
Duration: 45s
```

## Result
```
Workflow: completed
Total time: 265s (plus manual confirmation delay)
Files: none changed by workflow
Gates: 5/5 passed
```