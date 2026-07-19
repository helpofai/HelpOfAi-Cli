# Workflow Walkthrough: build-feature

## Input
```
hoa build feature "Add email/password auth with JWT"
```

## Phase 1: Design
```
Agent: architect
Input: feature description
Output: plan with 5 steps (migration, controller, service, middleware, tests)
Gate: review (critical=0) → PASS (0 critical)
Duration: 12s
```

## Phase 2: Implement
```
Engine: code
Input: plan from phase 1
Output: 6 files created, 1 file modified
Gate: quality (score=82 >= 60) → PASS
Duration: 45s
```

## Phase 3: Test
```
Agent: qa
Input: changed files
Output: 24 tests, 24 passed, 0 failed
Gate: test (all passed) → PASS
Duration: 22s
```

## Phase 4: Review
```
Agent: reviewer
Input: diff from phase 2 + test results from phase 3
Output: 0 critical, 2 minor (style)
Gate: review (critical=0) → PASS
Duration: 8s
```

## Result
```
Workflow: completed
Total time: 87s
Files: 6 created, 1 modified
Gates: 4/4 passed
```