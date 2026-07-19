# Project Health — CLI Dashboard Rendering

## Compact Mode (default)
```
Overall: 78/100  🟡
  Architecture  82 ↑  🟢
  Security      75 ↓  🟡 ⚠️
  Performance   90 →  🟢
  Coverage      85 ↑  🟢
  Docs          60 ↑  🟡
  Debt          70 ↓  🟡
```

## Detailed Mode (--verbose)
```
Project Health Dashboard
─────────────────────────
Architecture      82 ▲ (+2)  🟢  Good
  ✓ Layer separation: clean
  ✓ No circular dependencies
  ✗ 3 architecture violations in auth module

Security          75 ▼ (-5)  🟡  Needs attention
  ✗ 2 unfixed CVEs (high severity)
  ✓ No auth bypasses detected
  ✓ Compliance: SOC2 ready
...
```

## Machine Mode (--json)
Outputs structured JSON for programmatic consumption.
```
hoa health --json > health.json
```