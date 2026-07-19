# Agent System — Security Model

## Agent Isolation
Each agent runs in an isolated context. Agents cannot:
- Access other agents' in-memory state
- Spawn agents without master's permission
- Modify agent runtime configs
- Read other agents' tool bindings

## Tool Authorization
Tools are authorized at the agent level. An agent's tool list is
declared in its runtime config and enforced by the kernel.

## Audit Log
Every agent dispatch is logged:
```
{agent_id, task, spawned_by, start_time, end_time, tools_used, files_modified, result}
```

## Compliance
- All agent actions are traceable to a user request
- Agent communication is logged for replay debugging
- Sensitive data is masked in agent output logs