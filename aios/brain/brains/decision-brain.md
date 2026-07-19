# Decision Brain — AIOS-BRAIN-000005

ADR system. Every AIOS decision is recorded:
- Title (what was decided)
- Context (why it was needed)
- Alternatives considered
- Decision (what was chosen and why)
- Consequences (positive and negative)
- Timestamp + author (AIOS version or operator)

### Integration
- Future requests can query: "why was X done this way?"
- Decision Brain surfaces relevant ADRs to the planner
- ADRs survive project restarts (persisted via memory module)