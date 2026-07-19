# Risk Brain — AIOS-BRAIN-000009

Risk tracking database. Every identified risk is scored and tracked until
mitigated. The planner's risk_analysis contract reads this to produce its
blocking_steps[] list.

### Risk schema
- `severity`: critical | high | medium | low
- `likelihood`: 0.0-1.0
- `impact`: 0.0-1.0
- `mitigation`: strategy (accept|mitigate|transfer|avoid)
- `status`: open | mitigated | accepted | closed

### Integration
- Planner risk_analysis (AIOS-CONTRACT-000031) queries this per plan step
- Critical risks block plan approval until mitigated or accepted
- Risks with reversibility=0 trigger Law 4 rollback requirement