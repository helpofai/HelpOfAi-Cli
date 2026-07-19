# Profile Manager — CLI Integration Spec

The CLI manages AIOS profiles — named configurations that control which modules
load and how they behave.

## Profile File

`aios/.cache/profiles.json`

```json
{
  "active": "default",
  "profiles": {
    "default": {
      "modules": ["all"],
      "performance_budget": "standard",
      "log_level": "info"
    },
    "minimal": {
      "modules": ["kernel", "runtime"],
      "performance_budget": "minimal",
      "log_level": "warn"
    },
    "audit": {
      "modules": ["analysis", "security", "review"],
      "performance_budget": "thorough",
      "log_level": "debug"
    }
  }
}
```

## CLI Commands

```
hoa profile list          → list available profiles
hoa profile use <name>    → switch active profile
hoa profile create <name> → create new profile from current state
hoa profile delete <name> → remove a profile
```

## Profile-Aware Loading

When the CLI starts, it reads the active profile and loads only the modules
that match the profile's `modules` filter. A profile with `modules: ["all"]`
loads everything. A profile with `modules: ["kernel", "runtime"]` loads only
the core.

This enables use cases like:
- `minimal` profile for simple code review requests
- `audit` profile for security audits
- `default` profile for full feature development