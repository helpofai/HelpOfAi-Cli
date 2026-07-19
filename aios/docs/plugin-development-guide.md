# AIOS Plugin Development Guide

## Plugin Structure

```
plugins/my-plugin/
├── manifest.json          # Required: id, name, version, entry_point
├── main.spec.md           # Human-readable specification
├── contracts/             # Optional: capability contracts
│   └── my-capability.json
├── prompts/               # Optional: prompt templates
│   └── agent-prompt.md
├── workflows/             # Optional: workflow definitions
│   └── WORKFLOW-000100-my-workflow.json
└── CHANGELOG.md           # Recommended
```

## manifest.json

```json
{
  "id": "AIOS-PLUGIN-000001",
  "name": "My Custom Engine",
  "version": "1.0.0",
  "entry_point": "plugins/my-plugin/main.spec.md",
  "depends_on": ["AIOS-MODULE-000002"],
  "provides": [
    {"id": "AIOS-CAPABILITY-000200", "name": "custom_analysis"}
  ],
  "permissions": [
    {"scope": "filesystem", "trust_level": "sandboxed"}
  ]
}
```

## Restrictions (Engineering Law 16)
- Cannot modify kernel, runtime, or constitution
- Cannot override existing capabilities
- All file access is sandboxed to declared scope
- Plugin failures are caught and isolated — never crash the host

## Installation

```
# Copy to plugins/ directory
cp -r my-plugin/ plugins/

# CLI discovers it on next start
hoa module list
# → "My Custom Engine (AIOS-PLUGIN-000001)" should appear
```