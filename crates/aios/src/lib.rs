//! # helpofai-aios
//!
//! Parses and loads the AIOS (AI Software Engineering Operating System) module
//! system from the workspace `aios/` directory.
//!
//! ## Modules
//!
//! - `types` — Serde-backed Rust types for all AIOS JSON contracts
//! - `manifest` — Parse and validate a single `module.json`
//! - `registry` — Parse `modules.json`, `capabilities.json`, `dependencies.json`
//! - `loader` — Discover all modules, resolve dependencies, compute load order

pub mod agents;
pub mod brain;
pub mod constitution;
pub mod loader;
pub mod manifest;
pub mod registry;
pub mod types;
pub mod workflows;

// ── Re-exports ──────────────────────────────────────────────────

pub use agents::{AiosAgent, AiosAgentRegistry};
pub use brain::ProjectBrain;
pub use constitution::load_constitution_prompt;
pub use loader::{AiOsLoader, LoadedModule};
pub use manifest::{parse_manifest, validate_manifest};
pub use registry::{
    CapabilityRegistry, DependencyRegistry, ModuleRegistry, parse_all_registries,
    parse_capability_registry, parse_dependency_registry, parse_module_registry,
};
pub use types::{
    AgentDef, AiOSRoot, Capability, CapabilityDef, Constitution, ContractDef, DependencyEntry,
    IntegrationContract, ModuleCatalogEntry, ModuleCompatibility, ModuleId, ModuleManifest,
    PerformanceBudget, PermissionDef, Principle, WorkflowDef, module_id,
};
pub use workflows::AiosWorkflowRunner;

use std::path::{Path, PathBuf};

/// Resolves the AIOS root directory using global fallback strategies:
/// 1. `workspace_hint.join("aios")` if provided and `aios.json` exists
/// 2. Local CWD: `./aios/aios.json`
/// 3. Environment Variable `HELPOFAI_AIOS_DIR`
/// 4. User Home Directory: `~/.helpofai/aios/aios.json`
/// 5. Executable sibling directory: `<exe_dir>/aios/aios.json`
pub fn resolve_aios_root(workspace_hint: Option<&Path>) -> anyhow::Result<PathBuf> {
    if let Some(ws) = workspace_hint {
        let candidate = ws.join("aios");
        if candidate.join("aios.json").exists() {
            return Ok(candidate);
        }
    }

    let local = PathBuf::from("aios");
    if local.join("aios.json").exists() {
        return Ok(local);
    }

    if let Ok(env_path) = std::env::var("HELPOFAI_AIOS_DIR") {
        let candidate = PathBuf::from(&env_path);
        if candidate.join("aios.json").exists() {
            return Ok(candidate);
        }
    }

    if let Some(home) = dirs::home_dir() {
        let global = home.join(".helpofai").join("aios");
        if global.join("aios.json").exists() {
            return Ok(global);
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidate = parent.join("aios");
            if candidate.join("aios.json").exists() {
                return Ok(candidate);
            }
        }
    }

    anyhow::bail!(
        "AIOS root bundle (aios.json) not found in workspace (./aios), $HELPOFAI_AIOS_DIR, ~/.helpofai/aios, or next to binary executable."
    )
}
