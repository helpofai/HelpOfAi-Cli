# Workflow — API Contract

```
POST /workflow/run
{ "workflow": "build-feature", "inputs": {"description": "add auth"}, "auto_confirm": true }
→ { "task_id": "WF-20260719-001", "status": "running", "phases": [...] }

POST /workflow/status
{ "task_id": "WF-20260719-001" }
→ { "status": "completed", "current_phase": null, "gates_passed": 4, "gates_failed": 0 }

POST /workflow/cancel
{ "task_id": "WF-20260719-001" }
→ { "task_id": "WF-20260719-001", "status": "cancelled", "rollback_available": true }

POST /workflow/rollback
{ "task_id": "WF-20260719-001" }
→ { "task_id": "WF-20260719-001", "status": "rolling_back", "undo_count": 8 }

POST /workflow/list
{}
→ { "workflows": [{"id": "AIOS-WORKFLOW-000001", "name": "build-feature", "phases": 4}, ...] }
```