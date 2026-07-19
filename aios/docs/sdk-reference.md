# AIOS — SDK Reference

## Core Types

### Module
```json
{
  "id": "AIOS-MODULE-NNNNNN",
  "name": "string",
  "path": "PathString",
  "version": "semver",
  "status": "loaded|unloaded|failed",
  "capabilities": ["AIOS-CAPABILITY-NNNNNN"]
}
```

### Capability
```json
{
  "id": "AIOS-CAPABILITY-NNNNNN",
  "name": "string",
  "module_id": "AIOS-MODULE-NNNNNN",
  "contract_ref": "AIOS-CONTRACT-NNNNNN",
  "version": "semver"
}
```

### Agent
```json
{
  "id": "AIOS-AGENT-NNNNNN",
  "model": "string",
  "max_steps": "int",
  "capabilities": ["AIOS-CAPABILITY-NNNNNN"],
  "prompt_template": "PathString"
}
```

### Workflow
```json
{
  "id": "AIOS-WORKFLOW-NNNNNN",
  "name": "string",
  "phases": [
    {"agent_id": "AIOS-AGENT-NNNNNN", "gate": "review|test|quality|manual"},
    {"engine": "code|test|bug", "gate": "quality|test"}
  ],
  "rollback_policy": "full|bluegreen|none"
}
```

## API Endpoints

### Integration Layer
```
POST /integration/load       — load modules
GET  /integration/status     — module/capability/plugin status
GET  /registry/resolve       — resolve capability to module
POST /builder/classify       — classify request intent
POST /builder/build          — build workflow instance
POST /prompt/assemble        — assemble agent prompt
```

### Brain
```
POST /brain/index            — index files (full or incremental)
POST /brain/query            — query knowledge graph
POST /brain/context          — assemble context pack
GET  /brain/health           — brain health status
```

### Workflows
```
POST /workflow/run           — execute workflow
POST /workflow/status        — check workflow status
POST /workflow/cancel        — cancel running workflow
POST /workflow/rollback      — rollback completed workflow
```

## Error Handling

All API endpoints return errors in a standard format:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error description",
    "recovery": "Suggested recovery command"
  }
}
```