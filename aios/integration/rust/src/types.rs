use serde::{Deserialize, Serialize};

/// A module registered in the AIOS catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiosModule {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: String,
    pub status: Option<String>,
    pub capabilities: Vec<String>,
}

/// A capability provided by one or more modules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiosCapability {
    pub id: String,
    pub name: String,
    pub module_id: String,
    pub contract_ref: String,
}

/// Full registry payload from `aios/registry/modules.json`.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawRegistry {
    pub description: Option<String>,
    pub version: Option<String>,
    pub modules: std::collections::BTreeMap<String, RawModuleEntry>,
    pub installed_count: Option<u32>,
    pub total_count: Option<u32>,
    pub updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawModuleEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: String,
    pub status: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

/// Profile file at `aios/integration/data/profiles/{name}.json`.
#[derive(Debug, Deserialize)]
pub struct AiosProfile {
    pub profile: String,
    pub description: Option<String>,
    pub includes: Vec<String>,
    pub excludes: Option<Vec<String>>,
    pub load_timeout_ms: Option<u64>,
}

/// The result returned by the capability router.
#[derive(Debug, Clone, Serialize)]
pub struct RouterDecision {
    pub capability_id: String,
    pub module_id: String,
    pub module_name: String,
}

/// Summary of a completed load operation.
#[derive(Debug, Serialize)]
pub struct LoadSummary {
    pub total_modules: usize,
    pub loaded: usize,
    pub failed: usize,
    pub modules: Vec<LoadedModule>,
}

#[derive(Debug, Serialize)]
pub struct LoadedModule {
    pub id: String,
    pub name: String,
    pub status: String,
    pub capabilities: Vec<String>,
}