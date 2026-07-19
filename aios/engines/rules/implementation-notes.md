# Engines — Implementation Notes

## Engine Activation
Engines are loaded on demand when their capability is requested.
They remain loaded until the session ends or explicitly unloaded.

## Engine Isolation
Each engine runs in its own context. Engine A cannot:
- Read Engine B's in-memory state
- Modify Engine B's cache entries
- Access Engine B's filesystem scope

## Engine Versioning
Engines are versioned independently. The registry declares
compatibility constraints:
```
AIOS-MODULE-000009 (Code Engine) v1.0.0
  compatible_with: AIOS v1.x
  min_engine_version: 1.0.0
```

## Engine Dependencies
Engines can depend on other engines (e.g., Testing depends on Code).
Dependencies are declared in each engine's manifest.json:
```
"depends_on": ["AIOS-MODULE-000002", "AIOS-MODULE-000004"]
```
Optional dependencies are declared separately:
```
"optional_depends_on": ["AIOS-MODULE-000005"]
```