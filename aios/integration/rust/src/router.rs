use crate::error::AiosError;
use crate::registry::AiosRegistry;
use crate::types::RouterDecision;

/// Routes capability requests to the correct module.
pub struct AiosRouter {
    pub registry: AiosRegistry,
    pub loaded_modules: Vec<String>,
}

impl AiosRouter {
    pub fn new(registry: AiosRegistry, loaded_modules: Vec<String>) -> Self {
        Self {
            registry,
            loaded_modules,
        }
    }

    /// Route a capability request to the right module.
    pub fn route(&self, cap_id: &str) -> Result<RouterDecision, AiosError> {
        let decision = self.registry.resolve_capability(cap_id)?;
        if !self.loaded_modules.contains(&decision.module_id) {
            return Err(AiosError::ModuleNotLoaded {
                capability: cap_id.to_string(),
                module: decision.module_id.clone(),
            });
        }
        Ok(decision)
    }

    /// Classify a free-text request into a workflow name.
    pub fn classify(&self, request: &str) -> Option<String> {
        let lower = request.to_lowercase();
        let intents: &[(&[&str], &str)] = &[
            (&["build", "create", "add feature"], "build-feature"),
            (&["fix", "bug", "error", "crash", "broken"], "fix-bug"),
            (&["review", "inspect", "verify", "audit"], "review-code"),
            (&["refactor", "restructure", "reorganize"], "refactor"),
            (&["optimize", "performance", "speed up"], "optimize"),
            (&["release", "deploy", "ship", "version"], "release"),
        ];
        for (kws, wf) in intents {
            if kws.iter().any(|kw| lower.contains(kw)) {
                return Some(wf.to_string());
            }
        }
        None
    }
}