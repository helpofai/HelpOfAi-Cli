# Project Health — AIOS-ENTERPRISE-000050

Real-time project health dashboard. Each metric is scored 0-100 with trend
lines and blocking-issue tracking.

## Metrics

| Metric | Source | Weight |
|--------|--------|--------|
| Architecture Health | Analysis Engine | 25% |
| Security Posture | Security Platform | 20% |
| Performance | Performance Engine | 15% |
| Test Coverage | Testing Platform | 15% |
| Documentation | Documentation Engine | 10% |
| Technical Debt | Review Engine | 15% |

## CLI Commands
```
hoa health                → show all metrics
hoa health --refresh      → force re-scan
hoa health metric <name>  → show specific metric detail
```

Reports are rendered using `ARCHITECTURE_REPORT.md`, `SECURITY_REPORT.md`,
and `PERFORMANCE_REPORT.md` templates from `aios/templates/`.