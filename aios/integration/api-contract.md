# Integration — API Contract

## Loader API
```
POST /integration/load
{ "modules": ["AIOS-MODULE-000002", "AIOS-MODULE-000003"], "profile": null }
→ { "loaded": 2, "failed": 0, "status": "completed" }

GET /integration/status
{}
→ { "modules_loaded": 28, "capabilities_indexed": 34, "plugins_loaded": 0 }
```

## Registry API
```
GET /registry/capabilities
{ "module_id": null }
→ { "capabilities": [...] }

GET /registry/resolve
{ "capability_id": "AIOS-CAPABILITY-000010" }
→ { "module_id": "AIOS-MODULE-000002", "capability_name": "request_routing", "loaded": true }
```

## Builder API
```
POST /builder/classify
{ "request": "build auth feature" }
→ { "intent": "build-feature", "confidence": 1.0 }

POST /builder/build
{ "request": "build auth feature", "context": {} }
→ { "workflow_id": "AIOS-WORKFLOW-000001", "filled_inputs": {...} }
```

## Prompt API
```
POST /prompt/assemble
{ "agent_id": "AIOS-AGENT-000003", "plan_context": {}, "brain_context": {} }
→ { "prompt": "...", "token_count": 3240, "assembly_ms": 45 }
```