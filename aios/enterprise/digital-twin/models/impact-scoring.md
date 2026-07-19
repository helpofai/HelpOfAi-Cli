# Digital Twin — Impact Scoring Model

## Impact Levels
- **critical**: plan modifies immutable core (kernel, runtime, constitution) → blocked
- **high**: plan modifies files with active pending changes → conflict flag
- **medium**: plan modifies files with recent history → warning
- **low**: plan introduces new files only → no impact

## Scoring Formula
```
impact_score = (files_modified_weight × count) + (conflict_count × 10) + (critical_path_weight × path_bonus)
confidence    = 1.0 - (unindexed_files / total_files)
```

## Conflict Probability
```
P(conflict) = overlap_count / (overlap_count + unique_count)
where overlap = files both modified by pending + proposed plans
```