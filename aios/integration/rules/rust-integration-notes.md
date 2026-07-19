# Rust CLI Integration Notes

## Target Crate
Primary integration point: `crates/cli` in the HelpOfAi CLI workspace.

## Integration Points

### 1. Workspace Detection
```
On CLI startup:
  1. Check workspace root for aios/ directory
  2. If found: load AIOS integration module
  3. If not found: run without AIOS (fallback mode)
```

### 2. Command Routing
```
CLI command "hoa build feature" → 
  Detect AIOS active →
  Route to AIOS workflow builder →
  AIOS handles execution →
  Return result to CLI
```

### 3. Module Registry
```
CLI integration reads aios/registry/modules.json →
  Parse module list →
  Present as "hoa module list" →
  AIOS integration handles module lifecycle
```

### 4. Brain Cache
```
CLI integration provides the aios/.cache/brain/ directory →
  AIOS brain reads/writes from here →
  Cache persists across CLI restarts
```

### 5. Error Propagation
AIOS errors are propagated to the CLI as structured JSON:
```rust
pub struct AiosError {
    pub code: String,
    pub message: String,
    pub recovery: Option<String>,
}
```

### 6. Required Rust Crates
```toml
[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
walkdir = "2.0"       # for file scanning
sha2 = "0.10"         # for file hashing
glob = "0.3"          # for file pattern matching
```

### 7. Key Types
```rust
pub struct AiosModule {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub status: ModuleStatus,
    pub capabilities: Vec<Capability>,
}

pub enum ModuleStatus {
    Loaded,
    Unloaded,
    Failed(String),
}
```