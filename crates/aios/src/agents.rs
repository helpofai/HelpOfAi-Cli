//! # AIOS Agent Prompt Loader
//!
//! Loads AIOS agent specifications from `aios/agents/` and generates
//! system-prompt blocks that can be injected into sub-agent runs.

use std::collections::HashMap;
use std::path::Path;

use crate::types::{AgentDef, ModuleId};

/// A loaded AIOS agent ready for prompt injection.
#[derive(Debug, Clone)]
pub struct AiosAgent {
    pub spec: AgentDef,
    /// Markdown prompt excerpt from the agent's companion `.md` file,
    /// or a generated prompt from the JSON metadata.
    pub prompt_body: String,
    /// Whether the `.md` file existed on disk.
    pub has_md_prompt: bool,
}

/// Registry of all loaded AIOS agents, keyed by role name.
#[derive(Debug, Clone, Default)]
pub struct AiosAgentRegistry {
    pub agents: HashMap<String, AiosAgent>,
    pub by_id: HashMap<ModuleId, AiosAgent>,
}

impl AiosAgentRegistry {
    /// Load all AIOS agents from `aios/agents/`.
    pub fn load(aios_root: &Path) -> anyhow::Result<Self> {
        let agents_dir = aios_root.join("agents");
        let mut registry = Self::default();

        // Discover all AGENT-*.json files
        let dir_entries = std::fs::read_dir(&agents_dir)
            .map_err(|e| anyhow::anyhow!("cannot read agents dir {}: {e}", agents_dir.display()))?;

        for entry in dir_entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            // Only process AGENT-*.json files
            if !file_name.starts_with("AGENT-") || !file_name.ends_with(".json") {
                continue;
            }

            let raw = std::fs::read_to_string(&path)?;
            let spec: AgentDef = serde_json::from_str(&raw)
                .map_err(|e| anyhow::anyhow!("invalid agent spec {}: {e}", path.display()))?;

            // Try to find companion .md file
            let md_path = path.with_extension("md");
            let (prompt_body, has_md_prompt) = if md_path.exists() {
                let md = std::fs::read_to_string(&md_path)?;
                (md, true)
            } else {
                // Generate a prompt from the JSON metadata
                let generated = generate_agent_prompt(&spec);
                (generated, false)
            };

            let agent = AiosAgent {
                spec: spec.clone(),
                prompt_body,
                has_md_prompt,
            };

            registry.agents.insert(spec.role.clone(), agent.clone());
            registry.by_id.insert(spec.id.clone(), agent);
        }

        Ok(registry)
    }

    /// Resolve an agent by role name (e.g. "architect") or by ID.
    pub fn resolve(&self, query: &str) -> Option<&AiosAgent> {
        // Try exact role match first
        if let Some(agent) = self.agents.get(query) {
            return Some(agent);
        }
        // Try case-insensitive role match
        let lower = query.to_lowercase();
        for (role, agent) in &self.agents {
            if role.to_lowercase() == lower {
                return Some(agent);
            }
        }
        // Try by name (case-insensitive)
        for agent in self.agents.values() {
            if agent.spec.name.to_lowercase().contains(&lower) {
                return Some(agent);
            }
        }
        // Try by ID
        self.by_id.get(query)
    }

    /// Format an AIOS agent as a system-prompt injection block.
    pub fn format_prompt_injection(agent: &AiosAgent) -> String {
        let spec = &agent.spec;
        let mut block = String::new();

        block.push_str(&format!("## AIOS Agent: {} ({})\n\n", spec.name, spec.role));
        block.push_str(&format!(
            "**Role**: {} | **Domain**: {} | **Thinking**: {}\n\n",
            spec.role, spec.domain, spec.thinking_budget
        ));

        if !spec.required_capabilities.is_empty() {
            block.push_str(&format!(
                "**Required capabilities**: {}\n\n",
                spec.required_capabilities.join(", ")
            ));
        }

        if !spec.frameworks.is_empty() {
            block.push_str(&format!(
                "**Frameworks**: {}\n\n",
                spec.frameworks.join(", ")
            ));
        }

        if !spec.languages.is_empty() {
            block.push_str(&format!("**Languages**: {}\n\n", spec.languages.join(", ")));
        }

        // Append the agent's prompt body
        block.push_str(&agent.prompt_body);
        block.push('\n');

        block
    }
}

/// Generate a fallback system prompt from agent JSON metadata
/// when no companion `.md` file exists.
fn generate_agent_prompt(spec: &AgentDef) -> String {
    let mut prompt = String::new();

    prompt.push_str(&format!("# {} — AIOS Specialist Agent\n\n", spec.name));

    prompt.push_str(&format!(
        "You are the **{}** specialist within the AIOS engineering operating system.\n\n",
        spec.name
    ));

    prompt.push_str(&format!("{}", spec.description));

    if !spec.required_capabilities.is_empty() {
        prompt.push_str(&format!(
            "\n\nYour core capabilities are: {}.",
            spec.required_capabilities.join(", ")
        ));
    }

    if !spec.languages.is_empty() {
        prompt.push_str(&format!(
            " You specialize in: {}.",
            spec.languages.join(", ")
        ));
    }

    if !spec.frameworks.is_empty() {
        prompt.push_str(&format!(
            " You are proficient with: {}.",
            spec.frameworks.join(", ")
        ));
    }

    prompt.push_str("\n\nWork within the project's conventions. Produce production-quality output. Explain your decisions.");
    prompt.push('\n');

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_prompt_for_agent_without_md() {
        let spec = AgentDef {
            id: "AIOS-AGENT-000002".into(),
            name: "Architect Agent".into(),
            role: "architect".into(),
            version: "1.0.0".into(),
            description: "Architecture design and review.".into(),
            domain: "architecture".into(),
            languages: vec!["rust".into()],
            frameworks: vec![],
            required_capabilities: vec!["architecture_analysis".into()],
            ..Default::default()
        };
        let prompt = generate_agent_prompt(&spec);
        assert!(prompt.contains("Architect Agent"));
        assert!(prompt.contains("architecture_analysis"));
        assert!(prompt.contains("rust"));
    }
}
