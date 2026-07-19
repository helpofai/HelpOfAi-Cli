# Enterprise Features — Integration Test Fixtures

## Digital Twin — Mock Data
```
INPUT: plan_id = "test-plan-001"
EXPECTED: impact_report.files_created = 8, conflicts = 0, confidence > 0.7
```

## Timeline — Query Test
```
INPUT: hoa timeline --since 2026-07-01 --type rollback
EXPECTED: events[0].type = "rollback", events[0].summary contains "Migration"
```

## Decision Journal — Search Test
```
INPUT: hoa decision search "JWT"
EXPECTED: results[0].id = "ADR-000003", results[0].status = "proposed"
```

## Project Health — Scoring Test
```
INPUT: {architecture: 90, security: 80, performance: 85, coverage: 95, docs: 70, debt: 75}
EXPECTED: overall = 82.75 (green)
```

## Knowledge Base — Relevance Test
```
INPUT: search "state management"
EXPECTED: results[0].topic = "State management convention"
```

## Learning Engine — Pattern Test
```
INPUT: 5 bug types in 15 days
EXPECTED: confidence > 0.8, proposal generated
```

## Roadmap — Critical Path Test
```
INPUT: features [A(1wk), B(2wk,dep:A), C(1wk,dep:A)]
EXPECTED: critical_path = [A, B], duration = 3 weeks
```