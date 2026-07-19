# Capability Graph — View Filters

## Filter Types
- `module:<module_id>` — show only this module's capabilities
- `cap:<capability_id>` — trace route for a specific capability
- `provider:<module_id>` — inbound dependencies
- `dependent:<module_id>` — outbound dependents
- `unused` — capabilities with zero recent routing hits

## Export Formats
- **dot** — GraphViz for rendering
- **json** — structured graph data
- **mermaid** — Mermaid.js for documentation embedding

## Cache
Graph is cached at `aios/.cache/enterprise/capability-graph.json`.
Invalidated when registry changes.