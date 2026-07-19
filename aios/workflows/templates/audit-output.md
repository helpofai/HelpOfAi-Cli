# Audit — Example Output

## Project Health Report

| Metric | Score | Status |
|--------|-------|--------|
| Architecture Health | 8.2/10 | ✅ Good |
| Security Posture | 7.5/10 | ⚠️ 2 high findings |
| Performance | 9.0/10 | ✅ Good |
| Test Coverage | 85% | ✅ Good |
| Documentation | 6.0/10 | ❌ Needs improvement |

### Security Findings (Top 2)
1. **CRITICAL** — Dependency `lodash` v4.17.20 has CVE-2021-23337 → upgrade to 4.17.21
2. **HIGH** — API `/api/users` missing rate limiting → add throttle middleware

### Recommendations
1. Upgrade vulnerable dependencies (estimated: 30 min)
2. Add rate limiting to all public endpoints (estimated: 1 hr)
3. Generate API documentation with Documentation Engine