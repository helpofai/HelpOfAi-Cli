# Integration Scenario: Plugin Installation

## Context
Operator installs a community plugin.

## Steps
1. Operator copies plugin directory to `aios/plugins/example-plugin/`
2. Next CLI startup: integration scans `aios/plugins/`
3. Plugin loader:
   - Discovers `example-plugin/`
   - Reads manifest.json
   - Validates manifest (ID, version, permissions)
   - Checks capability uniqueness (AIOS-CAPABILITY-000200 not taken)
   - Registers plugin's capability with kernel
4. Status:
   - `hoa plugin list` → 1 plugin loaded
   - `hoa capability list` → 35 capabilities (34 built-in + 1 plugin)

## Uninstall
1. Operator: `hoa plugin unload AIOS-PLUGIN-000001`
2. Capability removed from kernel routing
3. Plugin directory NOT deleted (manual removal)
4. `hoa plugin list` → 0 plugins loaded