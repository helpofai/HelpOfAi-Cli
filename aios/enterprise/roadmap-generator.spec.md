# Roadmap Generator — AIOS-ENTERPRISE-000080

Generates a project roadmap from feature specs, dependencies, and capacity.
Estimates delivery timelines and flags risky dependencies or resource conflicts.

## Input
- List of feature IDs (AIOS-FEATURE-NNNNNN)
- Optional: team capacity (engineers, hours/week)
- Optional: start date

## Output
- Phased roadmap with start/end dates per phase
- Critical path identifying blocking features
- Risk flags for dependencies with high uncertainty
- Total duration estimate (min/max confidence range)

## Integration
Consumes the Feature Brain's dependency graph (AIOS-BRAIN-000003) to order
features and detect blocking chains. The planner's cost_estimation
(AIOS-CONTRACT-000032) provides per-feature effort estimates.