# Capability Graph — AIOS-ENTERPRISE-000020

Visual and queryable map of all AIOS capabilities, their module providers,
dependencies, and routing paths. Essential for understanding what AIOS can do.

## Views
- **Full graph**: all capabilities + modules + dependencies
- **Filtered by module**: show only one module's capabilities
- **Filtered by capability**: show which modules provide it
- **Route trace**: show how a request flows through the system

## Data Source
Reads from `aios/registry/capabilities.json` and `aios/registry/dependencies.json`
and renders the directed graph. Updates when modules are installed/removed.