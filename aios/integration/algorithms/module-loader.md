# Module Loader — Algorithm

## Load Sequence
```
1. Read aios/registry/modules.json → get all modules with load_order
2. Sort modules by load_order ascending
3. For each module:
   a. Check dependencies are loaded (skip if unresolved)
   b. Read module manifest (path/module.json)
   c. Validate manifest against SCHEMA-000011
   d. Register module with kernel (add capabilities to routing table)
   e. Load module contracts (path/contracts/*.json)
   f. Load module prompts (path/prompts/*.md) if present
   g. Set module status = "loaded"
4. Return list of loaded modules with status
```

## Dependency Resolution
```
for each module in load_order:
  deps = module.depends_on
  for dep in deps:
    if dep not in loaded_modules:
      FAIL: "Module {id} requires {dep} which is not loaded"
      Suggest: "Load {dep} first or change profile to include it"
  if dep not required (optional):
    WARN: "Optional dep {dep} not loaded — some capabilities unavailable"
```

## Error Recovery
- Skip unloadable modules — don't halt loading
- Log skipped modules with error details
- Allow operator to fix and reload via `hoa module reload`