# Registry Reader — CLI Integration Spec

The CLI reads three registry files at startup to build the capability graph.

## Files

| File | Purpose | Schema |
|------|---------|--------|
| `aios/registry/modules.json` | Module inventory + status | custom |
| `aios/registry/capabilities.json` | Capability→module mapping | custom |
| `aios/registry/dependencies.json` | Module dependency graph | custom |

## Cache Strategy

Registries are read once at startup and cached in memory. They rarely change
during a session. If a module is installed mid-session, the CLI re-reads
only that module's entry.

## CLI Commands

```
hoa module list       → reads modules.json, returns installed modules
hoa module info <id>  → reads specific module.json
hoa capability list   → reads capabilities.json
hoa capability resolve <cap> → finds providing module
hoa dependency tree <module> → resolves dep graph
```