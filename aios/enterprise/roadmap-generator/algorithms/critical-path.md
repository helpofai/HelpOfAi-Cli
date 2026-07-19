# Roadmap Generator — Critical Path Algorithm

The critical path identifies the longest chain of dependent features.

## Algorithm

```
1. Build dependency graph from feature specs
2. For each feature, compute:
   - earliest_start = max(earliest_finish of all dependencies)
   - earliest_finish = earliest_start + duration_estimate
3. Critical path = features where earliest_start + duration = project_duration
4. Slack = latest_finish - earliest_finish (0 for critical path features)
```

## Risk Flags
Features are flagged as risky if:
- Dependency count > 3
- Dependency has high uncertainty (>20% variance)
- Feature is on critical path AND has high complexity score
- Feature depends on unindexed external API

## Capacity Model
Default: 1 engineer = 5 features/week. Configurable via --capacity.
```
roadmap = features / (engineers * velocity)
where velocity = 5 features/week/engineer (adjustable)
```