# Engines — CLI Commands Reference

## Analysis
```
hoa analyze architecture [module]   → score architecture
hoa analyze security [target]       → security scan
hoa analyze all                     → full project analysis
```

## Code
```
hoa code generate --plan <plan_id>  → generate code from plan
hoa code preview --plan <plan_id>   → preview without writing
```

## Testing
```
hoa test run [path]                 → run tests for changed files
hoa test coverage [path]            → coverage report
hoa test watch                      → watch mode
```

## Review
```
hoa review [path]                   → review code changes
hoa review --safety                 → safety-only review
hoa review --style                  → style-only review
```

## Bug
```
hoa bug analyze <trace|log>         → analyze error evidence
hoa bug fix <analysis_id>           → generate fix
hoa bug patterns                    → list known bug patterns
```

## Security
```
hoa security scan [manifest]        → vulnerability scan
hoa security compliance             → compliance check
hoa security audit                  → full security audit
```

## DevOps
```
hoa devops plan --env staging       → deployment plan
hoa devops pipeline                 → generate CI/CD config
hoa devops status                   → environment status
```

## Performance
```
hoa perf profile [target]           → profile performance
hoa perf report                     → bottlenecks report
hoa perf budget                     → check against budget
```