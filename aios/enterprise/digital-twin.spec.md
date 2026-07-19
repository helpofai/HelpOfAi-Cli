# Digital Twin — AIOS-ENTERPRISE-000010

A live simulation of the project that runs plans against the knowledge graph
*before* touching real files. Predicts change impact, conflict probability,
and estimated effort with no side effects.

## Flow
1. Planner produces an ExecutionPlan
2. Digital Twin runs the plan against the knowledge graph (read-only)
3. Reports impact: files affected, conflicts predicted, effort estimate
4. Operator reviews before approving implementation

## Schema
- `impact_report`: files that would be created/modified/deleted
- `conflict_predictions`: files modified by multiple pending plans
- `effort_estimate`: tokens, files, time
- `confidence`: how reliable the prediction is (0.0-1.0)