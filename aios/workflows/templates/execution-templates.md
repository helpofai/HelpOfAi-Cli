# Workflow — Execution Templates

## build-feature Template
```
phases:
  1. design    (architect agent)     → gate: review (critical=0)
  2. implement (engine agent)        → gate: quality (score>=60)
  3. test      (qa agent)            → gate: test (all pass)
  4. review    (reviewer agent)      → gate: review (critical=0)
rollback: full (revert all files)
```

## fix-bug Template
```
phases:
  1. analyze   (bug engine)          → gate: auto (candidates found)
  2. fix       (code engine)         → gate: quality (score>=60)
  3. test      (qa agent)            → gate: test (all pass)
  4. review    (reviewer agent)      → gate: review (critical=0)
rollback: full (revert + revive old behavior)
```

## review-code Template
```
phases:
  1. scan      (review engine)       → gate: safety (no blockers)
  2. analyze   (analysis engine)     → gate: auto
  3. report    (generate report)     → gate: auto
rollback: none (read-only)
```

## refactor Template
```
phases:
  1. analyze   (analysis engine)     → gate: quality (score>=60)
  2. plan      (architect agent)     → gate: review (critical=0)
  3. implement (code engine)         → gate: quality (score>=65)
  4. test      (qa agent)            → gate: test (all pass)
  5. review    (reviewer agent)      → gate: review (critical=0)
rollback: full
```

## release Template
```
phases:
  1. test      (qa agent)            → gate: test (all pass)
  2. security  (security agent)      → gate: safety (no blockers)
  3. review    (reviewer agent)      → gate: review (critical=0)
  4. deploy    (devops engine)       → gate: manual
  5. verify    (qa agent)            → gate: test (all pass)
rollback: blue-green automatic
```