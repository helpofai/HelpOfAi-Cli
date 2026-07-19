# Profile Manager — Algorithm

## Profile Resolution
```
1. Look up profile in aios/integration/data/profiles/{name}.json
2. Build module set from includes:
   - Start with empty set
   - Add all modules from "includes" array
   - Remove all modules from "excludes" array
3. Verify module IDs exist in registry
4. Compute load plan:
   - Sort by registry load_order
   - Keep currently-loaded modules if they stay in set
   - Unload modules leaving the set
   - Load new modules entering the set
```

## Profile Switch
```
1. Current set: currently_loaded_modules
2. Target set: profile_modules
3. To unload: currently_loaded_modules - target_set
4. To load: target_set - currently_loaded_modules
5. Unload order: reverse load_order (dependents first)
6. Load order: registry load_order (dependencies first)
7. If any load fails: rollback to previous profile
```