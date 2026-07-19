# Workflow Walkthrough: fix-bug

## Input
```
hoa fix bug "Login fails with Google OAuth"
```

## Phase 1: Analyze
```
Engine: bug
Input: error description, stack trace
Output: root cause = OAuth callback URL mismatch, confidence 0.85
Gate: auto (candidates found) → PASS
Duration: 8s
```

## Phase 2: Fix
```
Engine: code
Input: analysis from phase 1
Output: 1 file modified (config/auth.php), rollback script generated
Gate: quality (score=78 >= 60) → PASS
Duration: 25s
```

## Phase 3: Test
```
Agent: qa
Input: changed files + analysis
Output: 3 new tests, 15/15 passed
Gate: test (all passed) → PASS
Duration: 15s
```

## Phase 4: Review
```
Agent: reviewer
Input: diff + test results
Output: 0 critical, 0 major, 1 minor
Gate: review (critical=0) → PASS
Duration: 5s
```

## Result
```
Workflow: completed
Total time: 53s
Files: 1 modified
Gates: 4/4 passed
```