# Digital Twin — Test Suite

## Test: Impact Scoring
```
INPUT: plan modifies 3 files, 0 conflicts
EXPECTED: impact_score = 30 (3 * 10 + 0 * 10 + 0)
```

## Test: Conflict Detection
```
INPUT: plan_a modifies auth.ts, plan_b modifies auth.ts
EXPECTED: conflict_level = 1, overlap_lines = "unknown" (needs diff)
```

## Test: Immutable Core Block
```
INPUT: plan modifies aios/kernel/module.json
EXPECTED: blocked = true, reason = "core module immutable"
```

## Test: Effort Estimation
```
INPUT: 8 new files, 0 external APIs
EXPECTED: effort_minutes = 40 (8 * 5 + 0 * 30)
```