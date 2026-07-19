# Registry Reader — Implementation Spec

## Core Function
```rust
fn resolve_capability(cap_id: &str) -> Result<(String, Capability), RegistryError>
```

## Data Structures
```rust
struct Registry {
    manifest: Manifest,
    modules: HashMap<String, ModuleEntry>,
    capabilities: HashMap<String, Capability>,
    dependencies: Vec<Dependency>,
}

struct ModuleEntry {
    id: String,
    name: String,
    path: PointPath,
    version: String,
    status: String,
    capabilities: Vec<String>,
    depends_on: Vec<String>,
}
```

## In-Memory Indexes
- `module_by_id: HashMap<String, ModuleEntry>` — O(1) lookup
- `module_by_name: HashMap<String, String>` — name → ID
- `capability_by_id: HashMap<String, Capability>` — O(1) lookup
- `capabilities_by_module: HashMap<String, Vec<String>>` — module ID → Vec<cap IDs>
- `modules_by_capability: HashMap<String, Vec<String>>` — cap ID → Vec<module IDs>

## Error Handling
```rust
enum RegistryError {
    RegistryNotFound,       // aios/registry/ missing
    RegistryInvalid(String), // JSON parse error
    CapabilityNotFound(String), // unknown cap_id
    ModuleNotFound(String), // unknown module_id
}
```