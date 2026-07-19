# Integration — Error Handling

## Loader Errors
| Code | Meaning | Recovery |
|------|---------|----------|
| LOADER_MANIFEST_MISSING | module.json not found | Skip module, log warning |
| LOADER_MANIFEST_INVALID | module.json fails schema | Skip module, log error |
| LOADER_DEP_NOT_FOUND | Required dependency missing | Skip module, suggest load order |
| LOADER_CYCLE | Circular dependency detected | Stop load, report cycle path |
| LOADER_DUPLICATE_ID | Two modules with same ID | Load first, skip second |

## Registry Reader Errors
| Code | Meaning | Recovery |
|------|---------|----------|
| REGISTRY_NOT_FOUND | aios/registry/ missing | Initialize registry |
| REGISTRY_INVALID | JSON parse failed | Restore from backup |
| CAPABILITY_NOT_FOUND | Capability ID unknown | List available capabilities |
| MODULE_NOT_LOADED | Module exists but not loaded | Suggest `hoa module load` |

## Workflow Builder Errors
| Code | Meaning | Recovery |
|------|---------|----------|
| BUILDER_INTENT_UNKNOWN | Could not classify intent | Ask operator for clarification |
| BUILDER_WORKFLOW_NOT_FOUND | Matched intent but no template | Create workflow template |

## Plugin Loader Errors
| Code | Meaning | Recovery |
|------|---------|----------|
| PLUGIN_MANIFEST_INVALID | Plugin manifest malformed | Skip plugin, log details |
| PLUGIN_PERMISSION_DENIED | Plugin requests full trust | Reject, sandbox not allowed |