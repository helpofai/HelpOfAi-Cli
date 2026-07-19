# Project Health — Scoring Algorithms

## Metric Scoring
Each metric is scored 0-100 using the formula:

### Architecture Health
```
score = (layer_violations * -10) + (circular_deps * -15) + (testability_score * 0.4) + 50
capped at 0-100
```

### Security Posture
```
score = (critical_findings * -25) + (high_findings * -10) + (medium_findings * -3) + 100
capped at 0-100
```

### Performance
```
score = (bottlenecks * -15) + (budget_violations * -10) + (p50_response_time_ok ? 20 : 0) + (p99_response_time_ok ? 15 : 0) + 50
capped at 0-100
```

### Test Coverage
```
score = coverage_percentage * 0.8 + (new_failures == 0 ? 20 : 0)
capped at 0-100
```

### Documentation
```
score = (public_api_documented_ratio * 0.4) + (readme_exists ? 20 : 0) + (changelog_updated ? 15 : 0) + (adrs_present ? 25 : 0)
capped at 0-100
```

### Technical Debt
```
score = 100 - (debt_items * 5) - (unresolved_critical * 15)
capped at 0-100
```

## Overall Health
```
overall = architecture * 0.25 + security * 0.20 + performance * 0.15 + coverage * 0.15 + docs * 0.10 + debt * 0.15
```

## Health Thresholds
- **Green** (>= 80): Good
- **Yellow** (>= 60): Needs attention
- **Red** (< 60): Critical