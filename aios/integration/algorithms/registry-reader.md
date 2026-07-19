# Registry Reader — Algorithm

## Registry Resolution
```
1. Load aios/registry/manifest.json → root registry metadata
2. Load aios/registry/modules.json → module catalog
3. Load aios/registry/capabilities.json → capability catalog
4. Load aios/registry/dependencies.json → dependency matrix
5. Build in-memory index:
   - module_by_id: fast lookup by module ID
   - module_by_name: fast lookup by name
   - capability_by_id: fast lookup by cap ID
   - capabilites_by_module: capabilities grouped by module
   - modules_by_capability: reverse map (capability → providing modules)
```

## Capability Resolution
```
resolve(capability_id):
  1. Look up capability_by_id[capability_id]
  2. Find providing module from modules_by_capability[capability_id]
  3. Check if module is loaded (status == "loaded")
  4. If loaded: return module_id + capability details
  5. If not loaded: return MODULE_NOT_LOADED error with load suggestion
```

## Cache Behavior
- Registry data is cached in memory after first load
- Refresh on `hoa registry refresh`
- Auto-refresh if module.json mtime changes