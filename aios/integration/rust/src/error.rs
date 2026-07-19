use std::fmt;

#[derive(Debug)]
pub enum AiosError {
    // --- Registry errors ---
    RegistryNotFound(String),
    RegistryParseFailed(String),
    /// The requested module id was not found in the registry.
    ModuleNotFound(String),
    /// The requested capability was not found.
    CapabilityNotFound(String),

    // --- Loader errors ---
    MissingDependency { module: String, dep: String },
    CircularDependency(Vec<String>),
    ProfileNotFound(String),

    // --- Router errors ---
    ModuleNotLoaded { capability: String, module: String },
    NoProviderFound(String),

    // --- I/O ---
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for AiosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegisterNotFound => write!(f, "Registry directory not found at aios/registry/"),
            Self::RegistryParseFailed(msg) => write!(f, "Failed to parse registry: {msg}"),
            Self::CapabilityNotFound(id) => write!(f, "Capability {id} not found"),
            Self::Dep { module, dep } => write!(f, "Module {module} depends on {dep} which is missing"),
            Self::DepCycle(chain) => write!(f, "Circular dependency: {}", chain.join(" → ")),
            Self::ProfileNotFound(name) => write!(f, "Profile {name} not found"),
            Self::ModuleNotLoaded { capability, module } => {
                write!(f, "Capability {capability} requires module {module} which is not loaded")
            }
            Self::NoProvider(id) => write!(f, "No module provides capability {id}"),
            Self(r) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl std::error::Error for AiosError {}

impl From<std::io::Error> for AiosError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for AiosError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}