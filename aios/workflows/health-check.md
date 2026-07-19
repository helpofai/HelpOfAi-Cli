# Workflow — Health Check

## Health Dimensions
| Dimension | Check | Scoring |
|-----------|-------|---------|
| Success Rate | % of workflows completed successfully | 0=poor, 100=perfect |
| Gate Pass Rate | % of gates passed on first try | 0=poor, 100=perfect |
| Rollback Rate | % of workflows requiring rollback | 0=poor, 100=perfect |

## CLI Output
```
hoa workflow health
→ Overall: 88/100 🟢
  Success:  92/100 🟢 (23/25 completed today)
  Gate Pass:85/100 🟢 (85% first-try pass rate)
  Rollback: 88/100 🟢 (3 rollbacks from 25 workflows)
```

## Per-Workflow Stats
```
build-feature: success=12/12, avg_time=45s, gate_pass=91%
fix-bug:       success=8/10,  avg_time=32s, gate_pass=80%
review-code:   success=5/5,   avg_time=12s, gate_pass=100%
```