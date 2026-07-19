# Learning Engine — Scoring Model

Proposal confidence score determines whether a pattern is surfaced to the operator.

## Formula
```
confidence = (observation_count * weight_type) * (time_window_factor) * (impact_severity)
```

## Factors
| Factor | Range | Description |
|--------|-------|-------------|
| observation_count | 0-N | How many times the pattern occurred |
| weight_type | 0.2-1.0 | Bug pattern=1.0, bottleneck=0.8, optimization=0.5, gap=0.3 |
| time_window_factor | 0.5-1.0 | Recent (1d)=1.0, This week=0.9, This month=0.7, Older=0.5 |
| impact_severity | 0.5-1.5 | Critical=1.5, High=1.0, Medium=0.7, Low=0.5 |

## Thresholds
- confidence >= 0.8: auto-surface to operator
- 0.5 <= confidence < 0.8: include in weekly digest
- confidence < 0.5: log only, don't surface

## Decay
Unsurfaced patterns decay by 10% per week. If confidence drops below 0.3,
the pattern is archived.