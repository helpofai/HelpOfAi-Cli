# Project Health — Trend Calculation

## Trend Direction
```
trend = current_score - previous_score
if trend > 2: "up" (↑ improving)
if trend in [-2, 2]: "stable" (→)
if trend < -2: "down" (↓ declining)
```

## Moving Average
```
ma_7d = sum(scores_last_7_days) / 7
ma_30d = sum(scores_last_30_days) / 30
```

## Forecasting
```
next_week = ma_7d + (ma_7d - ma_30d) * 0.3
if next_week < threshold → early warning flag
```

## Trend Visualization
```
Architecture:  82 ↑↑ (+5)
Security:      75 ↓ (-5) ⚠️
Performance:   90 → (0)
Coverage:      85 ↑ (+8)
Documentation: 60 ↑↑ (+10)
Debt:          70 ↓ (-3) ⚠️
```