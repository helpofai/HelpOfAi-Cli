# Learning Engine — Pattern Detection Rules

## Category: Bug Patterns
Detects when the same bug type appears ≥3 times in 30 days.
```
trigger: count(bug_type) >= 3 AND time_window <= 30 days
output: "Bug pattern detected: {type}. Suggestion: add static analysis rule."
```

## Category: Workflow Bottlenecks
Detects phases with >20% gate failure rate.
```
trigger: phase_gate_failures / phase_executions > 0.2
output: "Bottleneck: {phase} fails {rate}% of the time. Consider: {suggestions}"
```

## Category: Optimization Opportunities
Detects phases that are significantly slower than the performance budget.
```
trigger: phase_duration > performance_budget * 1.5
output: "{phase} exceeds budget by {overshoot}%. Consider: parallelize or cache."
```

## Category: Knowledge Gaps
Detects prompts that required the operator to clarify >2 times.
```
trigger: operator_clarifications > 2 PER same_topic
output: "Knowledge gap: {topic}. Suggested knowledge entry: {suggestion}"
```