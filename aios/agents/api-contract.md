# Agent System — API Contract

```
POST /agent/dispatch
{ "request": "build auth feature", "force_agent": null }
→ { "agent_id": "AIOS-AGENT-000001", "sub_agents": [...], "status": "running" }

POST /agent/status
{ "task_id": "..." }
→ { "status": "completed", "result_summary": "...", "duration_ms": 45000 }

POST /agent/health
{}
→ { "overall": 92, "dispatch_rate": 0.94, "queue_depth": 2, "fail_rate": 0.08 }

POST /agent/config
{ "agent_id": "AIOS-AGENT-000003", "max_steps": 20 }
→ { "agent_id": "AIOS-AGENT-000003", "config": {...}, "updated": true }
```