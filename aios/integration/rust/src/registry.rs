use std::collections::BTreeMap;
use std::path::Path;

use crate::error::AiosError;
use crate::types::{AiosCapability, AiosModule, RawRegistry, RouterDecision};

/// The parsed, in-memory module registry.
pub struct AiosRegistry {
    /// All known modules indexed by AIOS-MODULE-NNNNNN id.
    pub modules: BTreeMap<String, AiosModule>,
    /// All known capabilities (from capabilities.json).
    pub capabilities: BTreeMap<String, AiosCapability>,
}

impl AiosRegistry {
    /// Read the on-disk registry from `aios/registry/`.
    pub fn from_disk(project_root: &Path) -> Result<Self, AiosError> {
        let registry_dir = project_root.join("aios").join("registry");
        if !registry_dir.is_dir() {
            return Err(AiosError::RegistryNotFound(
                registry_dir.display().to_string(),
            ));
        }

        // --- modules.json ---
        let modules_path = registry_dir.join("modules.json");
        let raw: RawRegistry = serde_json::from_str(
            &std::fs::read_to_string(&modules_path)?,
        )?;

        let mut modules = BTreeMap::new();
        for (_name, entry) in raw.modules {
            modules.insert(
                entry.id.clone(),
                AiosModule {
                    id: entry.id,
                    name: entry.name,
                    version: entry.version,
                    path: entry.path,
                    status: entry.status,
                    capabilities: entry.capabilities,
                },
            );
        }

        // --- capabilities.json (optional) ---
        let caps_path = registry_dir.join("capabilities.json");
        let capabilities = if caps_path.is_file() {
            let raw_caps: BTreeMap<String, AiosCapability> = serde_json::from_str(
                &std::fs::read_to_string(&caps_path)?,
            )?;
            raw_caps
        } else {
            BTreeMap::new()
        };

        Ok(Self {
            modules,
            capabilities,
        })
    }

    /// Resolve a capability ID to the module that provides it.
    pub fn resolve_capability(
        &self,
        cap_id: &str,
    ) -> Result<RouterDecision, AiosError> {
        // Try exact match in capabilities catalog first.
        if let Some(cap) = self.capabilities.get(cap_id) {
            if let Some(mod_) = self.modules.get(&cap.module_id) {
                return Ok(RouterDecision {
                    capability_id: cap.id.clone(),
                    module_id: mod_.id.clone(),
                    module_name: mod_.name.clone(),
                });
            }
        }

        // Fallback: search module capability lists.
        for mod_ in self.modules.values() {
            if mod_.capabilities.iter().any(|c| c == cap_id) {
                return Ok(RouterDecision {
                    capability_id: cap_id.to_string(),
                    module_id: mod_.id.clone(),
                    module_name: mod_.name.clone(),
                });
            }
        }

        Err(AiosError::CapabilityNotFound(cap_id.into()))
    }

    /// List all loaded (status = "installed") modules.
    pub fn installed(&self) -> Vec<&AiosModule> {
        self.modules
            .values()
            .filter(|m| m.status.as_deref() == Some("installed"))
            .collect()
    }
}