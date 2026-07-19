# Review Engine — Review Scoring Algorithm

## Issue Severity Classification
```
CRITICAL: security vulnerability, data loss, broken functionality
MAJOR: incorrect behavior, performance regression, API violation
MINOR: style, naming, minor edge cases
NIT: optional improvements, suggestions
```

## Review Score
```
score = 100 - (critical * 30) - (major * 10) - (minor * 3) - (nit * 1)
capped at 0-100
```

## Pass/Fail
```
pass = critical == 0 AND major <= 3
```

## Comment Categories
| Category | Weight | Examples |
|----------|--------|----------|
| Security | 30 | XSS, injection, auth bypass, exposed secrets |
| Architecture | 25 | Layer violation, circular dep, tight coupling |
| Performance | 20 | N+1 query, memory leak, unnecessary allocation |
| Correctness | 15 | Race condition, null pointer, wrong logic |
| Style | 5 | Naming, formatting, idiomatic usage |
| Documentation | 5 | Missing docs, unclear comments |