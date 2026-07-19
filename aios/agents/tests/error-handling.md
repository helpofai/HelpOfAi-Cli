# Agent System — Error Handling

## Error Codes

| Code | Meaning | Action |
|------|---------|--------|
| AGENT_DISPATCH_FAILED | No agent matched request | Return to operator for clarification |
| AGENT_TIMEOUT | Agent exceeded max_steps | Terminate agent, return partial results |
| AGENT_TOOL_DENIED | Agent requested unauthorized tool | Block silently, log event |
| AGENT_CAPABILITY_MISSING | No tool for requested capability | Report to operator |
| AGENT_CONCURRENCY_LIMIT | Too many parallel agents | Queue request, report wait time |
| AGENT_ISOLATION_VIOLATION | Agent accessed protected resource | Block, log, potentially flag agent |

## Graceful Degradation
```
AGENT_TIMEOUT → return partial results, suggest splitting task
AGENT_DISPATCH_FAILED → fall back to master agent with reduced capability
AGENT_CONCURRENCY_LIMIT → queue with priority, notify operator
AGENT_TOOL_DENIED → silently log, continue without tool
```