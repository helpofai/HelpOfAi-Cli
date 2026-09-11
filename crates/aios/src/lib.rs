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
pub mod error_filter;
pub mod event_bus;
pub mod loader;
pub mod manifest;
pub mod registry;
pub mod types;
pub mod web_inspector;
pub mod workflows;
pub mod workspace_ops;

// ── Re-exports ──────────────────────────────────────────────────

pub use agents::{AiosAgent, AiosAgentRegistry};
pub use brain::ProjectBrain;
pub use constitution::load_constitution_prompt;
pub use event_bus::{AiosEvent, AiosEventReceiver, AiosEventSender, aios_channel};
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
pub use web_inspector::{
    ConsoleError, FileDiagnosticMatch, NetworkError, WebInspectEngine, WebInspectOptions,
    WebInspectReport, inspect_web_page,
};
pub use workflows::AiosWorkflowRunner;
pub use workspace_ops::{
    AiosWorkspaceOps, CollectedFile, OperationRecord, ProjectContextBundle, ProjectUpgradeAnalysis,
    TransactionJournal, UpgradeRecommendation,
};

use include_dir::{Dir, include_dir};
use std::path::{Path, PathBuf};

/// Built-in embedded AIOS bundle compiled directly into the binary.
pub static EMBEDDED_AIOS_BUNDLE: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../aios");

/// Unpacks the embedded AIOS architecture bundle into a target directory.
pub fn unpack_embedded_aios(target: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(target)?;
    EMBEDDED_AIOS_BUNDLE.extract(target)?;
    Ok(())
}

/// Resolves the AIOS root directory using global fallback strategies:
/// 1. `workspace_hint.join("aios")` if provided and `aios.json` exists
/// 2. Local CWD: `./aios/aios.json` (or any ancestor directory)
/// 3. Environment Variable `HELPOFAI_AIOS_DIR`
/// 4. User Home Directory: `~/.helpofai/aios/aios.json` (auto-unpacks embedded bundle if missing)
/// 5. Executable sibling directory: `<exe_dir>/aios/aios.json`
pub fn resolve_aios_root(workspace_hint: Option<&Path>) -> anyhow::Result<PathBuf> {
    if let Some(ws) = workspace_hint {
        for ancestor in ws.ancestors() {
            let dot_candidate = ancestor.join(".helpofai").join("aios");
            if dot_candidate.join("aios.json").exists() {
                return Ok(dot_candidate);
            }
            let candidate = ancestor.join("aios");
            if candidate.join("aios.json").exists() {
                return Ok(candidate);
            }
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            let dot_candidate = ancestor.join(".helpofai").join("aios");
            if dot_candidate.join("aios.json").exists() {
                return Ok(dot_candidate);
            }
            let candidate = ancestor.join("aios");
            if candidate.join("aios.json").exists() {
                return Ok(candidate);
            }
        }
    }

    if let Ok(env_path) = std::env::var("HELPOFAI_AIOS_DIR") {
        let candidate = PathBuf::from(&env_path);
        if candidate.join("aios.json").exists() {
            return Ok(candidate);
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

    if let Some(home) = dirs::home_dir() {
        let global = home.join(".helpofai").join("aios");
        if global.join("aios.json").exists() {
            return Ok(global);
        }

        // Auto-unpack embedded AIOS bundle so it works out-of-the-box for all users
        if unpack_embedded_aios(&global).is_ok() && global.join("aios.json").exists() {
            return Ok(global);
        }
    }

    anyhow::bail!(
        "AIOS root bundle (aios.json) not found in workspace (./aios), $HELPOFAI_AIOS_DIR, ~/.helpofai/aios, or next to binary executable."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_aios_bundle_contains_core_files() {
        assert!(EMBEDDED_AIOS_BUNDLE.get_file("aios.json").is_some());
        assert!(
            EMBEDDED_AIOS_BUNDLE
                .get_file("constitution/constitution.json")
                .is_some()
        );
        assert!(EMBEDDED_AIOS_BUNDLE.get_dir("registry").is_some());
        assert!(EMBEDDED_AIOS_BUNDLE.get_dir("workflows").is_some());
        assert!(EMBEDDED_AIOS_BUNDLE.get_dir("agents").is_some());
    }

    #[test]
    fn unpack_embedded_aios_extracts_cleanly() {
        let temp_dir = tempfile::tempdir().unwrap();
        let target = temp_dir.path().join("aios");
        unpack_embedded_aios(&target).unwrap();

        assert!(target.join("aios.json").exists());
        assert!(
            target
                .join("constitution")
                .join("constitution.json")
                .exists()
        );
        assert!(target.join("registry").join("modules.json").exists());
        assert!(target.join("workflows").exists());

        // resolve_aios_root with target's parent as workspace_hint
        let resolved = resolve_aios_root(Some(temp_dir.path())).unwrap();
        assert_eq!(resolved, target);
    }
}
