# Engines — Health Check Reference

## Analysis Health
```
hoa analyze health
→ Architecture index: fresh (5 min old)
→ Security DB: cached (2 hours old, 340 rules loaded)
```

## Code Health
```
hoa code health
→ Template cache: 12 templates loaded
→ Generation budget: 500ms / 256MB
```

## Testing Health
```
hoa test health
→ Framework cache: vitest, jest detected
→ Test cache: 240 test results cached (89% hit rate)
```

## Review Health
```
hoa review health
→ Rule count: 340 rules loaded
→ Avg review time: 120ms
```

## Bug Health
```
hoa bug health
→ Pattern DB: 12 known patterns
→ Avg analysis time: 80ms
```

## Security Health
```
hoa security health
→ CVE cache: 12,400 entries, 2 hours old
→ Compliance policies: 3 loaded (SOC2, GDPR, HIPAA)
```

## DevOps Health
```
hoa devops health
→ Pipeline templates: 4 (github_actions, gitlab_ci, jenkins, custom)
```

## Performance Health
```
hoa perf health
→ Budget: 200ms / 128MB
→ Avg profile time: 150ms
```