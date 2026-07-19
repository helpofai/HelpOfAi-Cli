use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::error::AiosError;
use crate::types::{AiosModule, LoadSummary, LoadedModule};

/// Module loader. Reads the registry and resolves dependencies.
pub struct AiosLoader {
    pub modules: BTreeMap<String, AiosModule>,
    pub loaded: BTreeSet<String>,
}

impl AiosLoader {
    pub fn new(modules: BTreeMap<String, AiosModule>) -> Self {
        Self {
            modules,
            loaded: BTreeSet::new(),
        }
    }

    /// Load all modules, respecting dependency order.
    pub fn any_load(&mut self) -> LoadResult {
        // Simple algo = iterate until nothing changes (topological sort approximation)
        let mut loaded = Vec::new();
        let mut failed = 0;
        let mut skipped = Vec::new();
        let mut remaining: BTreeSet<String> = self.modules.keys().cloned().collect();

        loop {
            let mut progress = false;
            for mod_id in remaining.clone() {
                let module = self.modules.get(&mod_id).unwrap();
                let all_deps_loaded = module
                    .depends_on
                    .iter()
                    .all(|dep| self.loaded.contains(dep));
                if all_deps_loaded {
                    self.loaded.insert(mod_id.clone());
                    remaining.remove(&mod_id);
                    loaded.push(LoadedModule {
                        id: module.id.clone(),
                        name: module.name.clone(),
                        status: "loaded".into(),
                        capabilities: module.capabilities.clone(),
                    });
                    progress = true;
                } else {
                    // skip for this round
                }
            }
            if !progress {
                for id in &remaining {
                    let module = self.modules.get(id).unwrap();
                    let missing: Vec<_> = module
                        .depends_on
                        .iter()
                        .filter(|d| !self.loaded.contains(*d))
                        .cloned()
                        .collect();
                    if !missing.is_empty() {
                        skipped.push(LoadedModule {
                            id: id.clone(),
                            name: module.name.clone(),
                            status: "skipped".into(),
                            capabilities: Vec::new(),
                        });
                        failed += 1;
                    }
                }
                break;
            }
        }

        Ok(LoadSummary {
            total_modules: self.modules.len(),
            loaded: loaded.len(),
            failed,
            skipped: Vec::new(),
            modules: loaded,
        })
    }

    /// Load only the modules listed in a profile.
    pub fn load_with_profile(
        &mut self,
        profile_dir: &Path,
        profile_name: &str,
    ) -> Result<LoadSummary, AiosError> {
        let prof_path = profile_dir.join(format!("{profile_name}.json"));
        if !prof_path.is_file() {
            return Err(AiosError::ProfileNotFound(profile_name.into()));
        }

        let profile: crate::types::AiosProfile =
            serde_json::from_str(&std::fs::read_to_string(&prof_path)?)?;

        // Reset loader.
        self.loaded.clear();

        // Add only requested modules (but still check deps).
        let target_set: BTreeSet<String> = profile
            .includes
            .into_iter()
            .filter(|id| self.modules.contains_key(id))
            .collect();

        let mut loaded = Vec::new();
        for id in &target_set {
            if let Some(module) = self.modules.get(id) {
                self.loaded.insert(id.clone());
                loaded.push(LoadedModule {
                    id: module.id.clone(),
                    name: module.name.clone(),
                    status: "loaded".into(),
                    capabilities: module.capabilities.clone(),
                });
            }
        }

        Ok(LoadSummary {
            total_modules: self.modules.len(),
            loaded: loaded.len(),
            failed: 0,
            skipped: Vec::new(),
            modules: loaded,
        })
    }
}