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
pub mod constitution;
pub mod loader;
pub mod manifest;
pub mod registry;
pub mod types;
pub mod workflows;

// ── Re-exports ──────────────────────────────────────────────────

pub use agents::{AiosAgent, AiosAgentRegistry};
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
