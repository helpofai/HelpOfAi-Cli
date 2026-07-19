//! # Registry parsers
//!
//! Reads the three core AIOS registry files:
//!
//! - `registry/modules.json`  → `ModuleRegistry`
//! - `registry/capabilities.json` → `CapabilityRegistry`
//! - `registry/dependencies.json` → `DependencyRegistry`
//!
//! Each parser returns a structured Rust value with public fields
//! suitable for CLI display and loader consumption.

use std::collections::HashMap;
use std::path::Path;

use crate::types::{Capability, DependencyEntry, ModuleCatalogEntry, ModuleId};

// ── Module Registry ─────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModuleRegistry {
    /// Description from the JSON header.
    pub description: String,
    pub version: String,
    /// Every module catalogued, keyed by its registry key (e.g. "kernel").
    pub modules: HashMap<String, ModuleCatalogEntry>,
    pub installed_count: usize,
    pub total_count: usize,
    pub updated: String,
}

pub fn parse_module_registry(path: &Path) -> anyhow::Result<ModuleRegistry> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;

    #[derive(serde::Deserialize)]
    struct Raw {
        #[serde(default)]
        description: String,
        #[serde(default)]
        version: String,
        #[serde(default)]
        modules: HashMap<String, ModuleCatalogEntry>,
        #[serde(default)]
        installed_count: usize,
        #[serde(default)]
        total_count: usize,
        #[serde(default)]
        updated: String,
    }

    let raw: Raw = serde_json::from_str(&raw).map_err(|e| {
        anyhow::anyhow!(
            "{} is not a valid AIOS module registry: {e}",
            path.display()
        )
    })?;

    Ok(ModuleRegistry {
        description: raw.description,
        version: raw.version,
        modules: raw.modules,
        installed_count: raw.installed_count,
        total_count: raw.total_count,
        updated: raw.updated,
    })
}

// ── Capability Registry ─────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct CapabilityRegistry {
    pub description: String,
    pub version: String,
    /// Keyed by capability name (e.g. "request_routing"), not by ID.
    pub capabilities: HashMap<String, Capability>,
    pub updated: String,
}

impl CapabilityRegistry {
    /// Resolve a capability ID → (provider_name, module_id).
    pub fn resolve(&self, capability_id: &str) -> Option<(&str, &ModuleId)> {
        for cap in self.capabilities.values() {
            if cap.id == capability_id {
                return Some((&cap.provider, &cap.module_id));
            }
        }
        None
    }

    /// List all capabilities as `(name, id, module_id)`.
    pub fn list(&self) -> Vec<(&str, &ModuleId, &ModuleId)> {
        self.capabilities
            .iter()
            .map(|(name, cap)| (name.as_str(), &cap.id, &cap.module_id))
            .collect()
    }
}

pub fn parse_capability_registry(path: &Path) -> anyhow::Result<CapabilityRegistry> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;

    #[derive(serde::Deserialize)]
    struct Raw {
        #[serde(default)]
        description: String,
        #[serde(default)]
        version: String,
        #[serde(default)]
        capabilities: HashMap<String, Capability>,
        #[serde(default)]
        updated: String,
    }

    let mut raw: Raw = serde_json::from_str(&raw).map_err(|e| {
        anyhow::anyhow!(
            "{} is not a valid AIOS capability registry: {e}",
            path.display()
        )
    })?;

    for (name, cap) in &mut raw.capabilities {
        cap.name = name.clone();
    }

    Ok(CapabilityRegistry {
        description: raw.description,
        version: raw.version,
        capabilities: raw.capabilities,
        updated: raw.updated,
    })
}

// ── Dependency Registry ─────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct DependencyRegistry {
    pub description: String,
    pub version: String,
    /// Keyed by module ID (e.g. "AIOS-MODULE-000002").
    pub dependencies: HashMap<ModuleId, DependencyEntry>,
    pub resolver_version: String,
    pub updated: String,
}

impl DependencyRegistry {
    /// Return direct dependencies of a module (empty slice if not found).
    pub fn depends_on(&self, module_id: &str) -> &[String] {
        self.dependencies
            .get(module_id)
            .map(|e| e.depends_on.as_slice())
            .unwrap_or(&[])
    }

    /// Whether a module declares itself as optional.
    pub fn is_optional(&self, module_id: &str) -> bool {
        self.dependencies
            .get(module_id)
            .map(|e| e.optional)
            .unwrap_or(false)
    }
}

pub fn parse_dependency_registry(path: &Path) -> anyhow::Result<DependencyRegistry> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;

    #[derive(serde::Deserialize)]
    struct Raw {
        #[serde(default)]
        description: String,
        #[serde(default)]
        version: String,
        #[serde(default)]
        dependencies: HashMap<ModuleId, DependencyEntry>,
        #[serde(default)]
        resolver_version: String,
        #[serde(default)]
        updated: String,
    }

    let raw: Raw = serde_json::from_str(&raw).map_err(|e| {
        anyhow::anyhow!(
            "{} is not a valid AIOS dependency registry: {e}",
            path.display()
        )
    })?;

    Ok(DependencyRegistry {
        description: raw.description,
        version: raw.version,
        dependencies: raw.dependencies,
        resolver_version: raw.resolver_version,
        updated: raw.updated,
    })
}

/// Convenience: read all three registries from an `aios/registry/` directory.
pub fn parse_all_registries(
    registry_dir: &Path,
) -> anyhow::Result<(ModuleRegistry, CapabilityRegistry, DependencyRegistry)> {
    let modules = parse_module_registry(&registry_dir.join("modules.json"))?;
    let capabilities = parse_capability_registry(&registry_dir.join("capabilities.json"))?;
    let dependencies = parse_dependency_registry(&registry_dir.join("dependencies.json"))?;
    Ok((modules, capabilities, dependencies))
}
