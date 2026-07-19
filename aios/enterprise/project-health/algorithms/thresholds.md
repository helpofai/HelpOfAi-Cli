# Project Health — Threshold Configuration

## Hard Thresholds (cannot be changed by operator)
```
architecture_min:    50  (below = architecture needs immediate attention)
security_min:        50  (below = deployment blocked)
test_coverage_min:   30  (below = module flagged as "untrusted")
```

## Soft Thresholds (configurable)
Configured in aios/.cache/enterprise/health-config.json:
```json
{
  "warning_thresholds": {
    "architecture": 60,
    "security": 60,
    "performance": 60,
    "coverage": 60,
    "documentation": 40,
    "debt": 50
  },
  "good_thresholds": {
    "architecture": 80,
    "security": 80,
    "performance": 80,
    "coverage": 80,
    "documentation": 60,
    "debt": 70
  }
}
```

## Threshold Effects
- Below warning: yellow status, surfaces in CLI output
- Below hard minimum: red status, blocks deployment
- Below hard minimum for 7 days: automatic issue logged