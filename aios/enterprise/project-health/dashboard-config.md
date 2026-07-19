# Project Health — Dashboard Configuration

## Refresh
- Manual: `hoa health --refresh`
- Auto: every 4 hours when CLI is active
- On demand: triggered after `build-feature` or `release` workflows complete

## Metric Weights
Configurable in `aios/.cache/enterprise/health-config.json`

```json
{
  "weights": {
    "architecture_health": 25,
    "security_posture": 20,
    "performance": 15,
    "test_coverage": 15,
    "documentation": 10,
    "technical_debt": 15
  },
  "thresholds": {
    "green": 80,
    "yellow": 60
  },
  "blocking_critical": true
}
```

## Display
Rendered using these templates:
- Overall: `ARCHITECTURE_REPORT.md` (aios/templates/)
- Per metric: `SECURITY_REPORT.md`, `PERFORMANCE_REPORT.md`
- Trend sparklines: in CLI output (compact bar graph)