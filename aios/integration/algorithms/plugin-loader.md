# Plugin Loader — Algorithm

## Load Sequence
```
1. Scan aios/plugins/ directory for subdirectories
2. For each plugin directory:
   a. Look for manifest.json
   b. Validate manifest against plugin schema:
      - id: string, matches AIOS-PLUGIN-NNNNNN
      - name: string
      - version: semver
      - entry_point: existing file path
      - depends_on: valid module IDs
      - permissions: array of {scope, trust_level}
   c. Check permissions against allowed:
      - trust_level must be in ["sandboxed"] (no "full" allowed)
      - scope must be declared
   d. Register with kernel:
      - Add plugin's capabilities
      - Set trust boundaries
   e. Set plugin status = "loaded"
3. Return list of loaded plugins with status
```

## Unload Sequence
```
1. Remove plugin's capabilities from kernel routing table
2. Clear plugin's cache entries
3. Set plugin status = "unloaded"
4. Plugin directory is NOT deleted (operator must remove manually)
```