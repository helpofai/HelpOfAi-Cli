//! # AIOS Module Loader
//!
//! Discovers all AIOS modules from the `aios/` directory, parses each
//! module manifest, resolves dependencies, and computes a valid load order
//! (topological sort with optional-module skipping).

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use crate::manifest::{parse_manifest, validate_manifest};
use crate::registry::{DependencyRegistry, parse_all_registries};
use crate::types::ModuleManifest;

/// A module that has been discovered, parsed, and is ready to load.
#[derive(Debug, Clone)]
pub struct LoadedModule {
    /// The manifest parsed from `module.json`.
    pub manifest: ModuleManifest,
    /// Absolute path to the module directory.
    pub dir: PathBuf,
    /// Whether this module's dependencies are fully satisfied.
    pub dependencies_ok: bool,
}

/// The top-level AIOS loader.
pub struct AiOsLoader {
    /// Absolute path to the `aios/` root directory.
    root: PathBuf,
}

impl AiOsLoader {
    /// Create a loader that reads from the given `aios/` root.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Discover all modules listed in the module registry and parse
    /// their manifests. Returns loaded modules in registry order.
    pub fn discover_modules(&self) -> anyhow::Result<Vec<LoadedModule>> {
        let registry_dir = self.root.join("registry");
        let (module_registry, _cap_registry, _dep_registry) = parse_all_registries(&registry_dir)?;

        let mut loaded = Vec::new();

        for entry in module_registry.modules.values() {
            // Skip the root entry and constitution (they're not loadable modules)
            if entry.id.starts_with("AIOS-ROOT") || entry.id.starts_with("AIOS-CONST") {
                continue;
            }

            let module_dir = self.root.join(&entry.path);
            let manifest_path = if module_dir.is_dir() {
                module_dir.join("module.json")
            } else {
                // Some entries point directly to a JSON file
                self.root.join(&entry.path)
            };

            if !manifest_path.exists() {
                // Module declared in registry but no manifest on disk — skip gracefully
                continue;
            }

            match parse_manifest(&manifest_path) {
                Ok(manifest) => {
                    if let Err(e) = validate_manifest(&manifest) {
                        eprintln!(
                            "AIOS: skipping invalid manifest {}: {e}",
                            manifest_path.display()
                        );
                        continue;
                    }
                    let dir = if module_dir.is_dir() {
                        module_dir
                    } else {
                        manifest_path.parent().unwrap_or(&self.root).to_path_buf()
                    };
                    loaded.push(LoadedModule {
                        manifest,
                        dir,
                        dependencies_ok: false,
                    });
                }
                Err(e) => {
                    eprintln!(
                        "AIOS: cannot parse manifest {}: {e}",
                        manifest_path.display()
                    );
                }
            }
        }

        Ok(loaded)
    }

    /// Compute a valid load order for the given modules, using the
    /// dependency registry to resolve dependencies.
    ///
    /// The algorithm is a Kahn-style topological sort:
    /// 1. Build adjacency: for each module, its dependants (modules that need it).
    /// 2. Start with modules that have zero unsatisfied dependencies.
    /// 3. Process each, removing it from dependants' pending counts.
    /// 4. Any modules left unprocessed are either optional or have unresolved cycles.
    pub fn compute_load_order(
        modules: &[LoadedModule],
        dep_registry: &DependencyRegistry,
    ) -> Vec<usize> {
        // Map module ID → index in the modules slice
        let id_to_index: HashMap<&str, usize> = modules
            .iter()
            .enumerate()
            .map(|(i, m)| (m.manifest.id.as_str(), i))
            .collect();

        // For each module index, count how many of its dependencies are
        // NOT present in the loaded set (unsatisfied).
        let mut unsatisfied: Vec<usize> = modules
            .iter()
            .map(|m| {
                dep_registry
                    .depends_on(&m.manifest.id)
                    .iter()
                    .filter(|dep_id| !id_to_index.contains_key(dep_id.as_str()))
                    .count()
            })
            .collect();

        // Build reverse edges: for each module A, which loaded modules
        // depend on A?
        let mut dependants: Vec<Vec<usize>> = vec![Vec::new(); modules.len()];
        for (i, m) in modules.iter().enumerate() {
            for dep_id in dep_registry.depends_on(&m.manifest.id) {
                if let Some(&j) = id_to_index.get(dep_id.as_str()) {
                    dependants[j].push(i);
                }
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<usize> = (0..modules.len())
            .filter(|&i| unsatisfied[i] == 0)
            .collect();

        let mut order = Vec::new();
        let mut processed = HashSet::new();

        while let Some(i) = queue.pop_front() {
            if processed.contains(&i) {
                continue;
            }
            processed.insert(i);
            order.push(i);

            for &dep in &dependants[i] {
                if unsatisfied[dep] > 0 {
                    unsatisfied[dep] -= 1;
                }
                if unsatisfied[dep] == 0 && !processed.contains(&dep) {
                    queue.push_back(dep);
                }
            }
        }

        // Append any remaining modules (optional or cyclic) at the end
        for i in 0..modules.len() {
            if !processed.contains(&i) {
                order.push(i);
            }
        }

        order
    }

    /// Full pipeline: discover modules, compute load order, return both.
    pub fn load(&self) -> anyhow::Result<(Vec<LoadedModule>, Vec<usize>, DependencyRegistry)> {
        let registry_dir = self.root.join("registry");
        let (_mod_reg, _cap_reg, dep_registry) = parse_all_registries(&registry_dir)?;

        let modules = self.discover_modules()?;
        let order = Self::compute_load_order(&modules, &dep_registry);

        Ok((modules, order, dep_registry))
    }
}
