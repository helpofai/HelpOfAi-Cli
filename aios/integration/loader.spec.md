# Module Loader — CLI Integration Spec

The CLI discovers and loads AIOS modules by reading the module registry.

## Discovery Protocol

1. Read `aios/registry/modules.json` for the full module list.
2. For each module with `status: installed`, read `aios/{module}/module.json`.
3. Validate the manifest against `aios/schemas/SCHEMA-000011-module.json`.
4. Resolve dependencies via `aios/registry/dependencies.json`.
5. Load the module's contracts from `aios/{module}/contracts/*.json`.
6. Register capabilities from `module.json.provides[]` into the runtime capability map.

## Load Order

Modules are loaded in `load_order` ascending. Required modules (kernel, runtime)
must load before optional modules. If a required module fails to load, AIOS must
not proceed.

## Resolution

`aios/registry/capabilities.json` maps capability IDs to module IDs. The CLI
uses this to route requests:
```
request → kernel → capability IDs → capability_registry → module_ids → load modules
```