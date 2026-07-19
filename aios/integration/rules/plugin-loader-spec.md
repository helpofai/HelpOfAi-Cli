# Plugin Loader — Implementation Spec

## Core Function
```rust
fn load_plugin(plugin_dir: &Path) -> Result<Plugin, PluginError>
fn unload_plugin(plugin_id: &str) -> Result<(), PluginError>
fn list_plugins() -> Vec<PluginSummary>
```

## Plugin Manifest Schema
```rust
struct PluginManifest {
    id: String,                  // AIOS-PLUGIN-NNNNNN
    name: String,
    version: String,             // semver
    entry_point: String,         // relative path
    depends_on: Vec<String>,     // module IDs
    provides: Vec<CapabilityRef>,
    permissions: Vec<Permission>,
}

struct Permission {
    scope: String,               // "filesystem", "network", "memory"
    trust_level: String,         // "sandboxed" only
}
```

## Validation Rules
```
- id must match /AIOS-PLUGIN-\d{6}/
- version must be valid semver
- entry_point must be existing file under plugin_dir
- depends_on must reference valid module IDs from registry
- trust_level must be "sandboxed" (full not allowed)
- provides.capability_id must be unique (no overlap with built-in caps)
```

## Security Constraints
```rust
fn check_permissions(plugin: &Plugin) -> Result<(), PluginError> {
    for perm in &plugin.manifest.permissions {
        match (perm.scope.as_str(), perm.trust_level.as_str()) {
            ("filesystem", "sandboxed") => continue,
            ("network", "sandboxed") => continue,
            (scope, "full") => return Err(PluginError::PermissionDenied(
                format!("Plugin cannot have full trust for {}", scope))),
            _ => return Err(PluginError::PermissionDenied(
                format!("Unknown permission scope or level: {}/{}", scope, level))),
        }
    }
    Ok(())
}
```