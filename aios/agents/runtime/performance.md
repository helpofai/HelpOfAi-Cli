# Agent System — Performance Budget

## Dispatch Budget
```
Agent dispatch: 50ms max
Intent classification: 30ms max
Sub-agent spawn: 100ms max
Result synthesis: 200ms max
```

## Memory Budget
```
Master agent: 128MB
Specialist agent: 64MB each
Agent queue: 64MB
Total agent system: 256MB
```

## Concurrency
```
Max parallel agents: 5
Max queue depth: 20
Max agent lifetime: 300s
```