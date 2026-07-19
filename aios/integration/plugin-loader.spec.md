# Plugin Loader — CLI Integration Spec

The CLI discovers and loads AIOS plugins dynamically.

## Discovery

Plugins live in `plugins/` at the workspace root. Each plugin has:
```
plugins/{plugin-name}/
  manifest.json   → required: id, name, version, entry_point
  contracts/      → optional: contract definitions
  prompts/        → optional: prompt templates
  workflows/      → optional: workflow definitions
```

## Validation

On discovery, the CLI validates:
1. `manifest.json` against a plugin schema
2. Dependencies are installed (or installable)
3. No conflicts with loaded modules
4. Plugin does not declare `conflicts_with` a loaded module

## Security

Plugins are sandboxed per their declared `permissions`. A plugin cannot:
- Modify core modules (kernel, runtime, constitution) — Engineering Law 16
- Access files outside its declared scope
- Register capabilities that conflict with existing ones

## Lifecycle

```
plugin installed → CLI discovers on next start
  → validate manifest
  → resolve dependencies
  → register capabilities
  → ready for routing
```