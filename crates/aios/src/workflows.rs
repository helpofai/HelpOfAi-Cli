//! # AIOS Workflow Execution Engine
//!
//! Orchestrates the lifecycle execution of AIOS workflows. Reads workflows from
//! `aios/workflows/`, compiles prompts for each phase, checks agent capabilities,
//! and runs sequential workflow phases with gating and rollback capability.

use anyhow::{Context, Result, bail};
use std::path::PathBuf;

use crate::agents::{AiosAgent, AiosAgentRegistry};
use crate::constitution::load_constitution_prompt;
use crate::registry::parse_all_registries;
use crate::types::{WorkflowDef, WorkflowPhase};

/// Orchestrates stateful execution of a workflow lifecycle.
pub struct AiosWorkflowRunner {
    aios_root: PathBuf,
    agent_registry: AiosAgentRegistry,
}

impl AiosWorkflowRunner {
    /// Create a new workflow runner pointing to the given `aios/` root.
    pub fn new(aios_root: impl Into<PathBuf>) -> Result<Self> {
        let aios_root = aios_root.into();
        let agent_registry = AiosAgentRegistry::load(&aios_root)
            .context("failed to load agent registry for workflow runner")?;

        Ok(Self {
            aios_root,
            agent_registry,
        })
    }

    /// Load a workflow definition from the workspace.
    pub fn load_workflow(&self, workflow_name: &str) -> Result<WorkflowDef> {
        let workflows_dir = self.aios_root.join("workflows");

        // Match by filename pattern: WORKFLOW-*-<name>.json or exact id
        let dir_entries = std::fs::read_dir(&workflows_dir).map_err(|e| {
            anyhow::anyhow!("cannot read workflows dir {}: {e}", workflows_dir.display())
        })?;

        for entry in dir_entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if !file_name.ends_with(".json") {
                continue;
            }

            let raw = std::fs::read_to_string(&path)?;
            let w_def: WorkflowDef = serde_json::from_str(&raw)
                .map_err(|e| anyhow::anyhow!("invalid workflow JSON in {}: {e}", path.display()))?;

            if w_def.name == workflow_name
                || w_def.id == workflow_name
                || file_name.contains(workflow_name)
            {
                return Ok(w_def);
            }
        }

        bail!("Workflow '{}' not found in registry.", workflow_name)
    }

    /// List all workflows in the system.
    pub fn list_workflows(&self) -> Result<Vec<WorkflowDef>> {
        let workflows_dir = self.aios_root.join("workflows");
        let mut list = Vec::new();

        let dir_entries = std::fs::read_dir(&workflows_dir).map_err(|e| {
            anyhow::anyhow!("cannot read workflows dir {}: {e}", workflows_dir.display())
        })?;

        for entry in dir_entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if !file_name.ends_with(".json") {
                continue;
            }

            if let Ok(raw) = std::fs::read_to_string(&path) {
                if let Ok(w_def) = serde_json::from_str::<WorkflowDef>(&raw) {
                    list.push(w_def);
                }
            }
        }

        Ok(list)
    }

    /// Compile a target system prompt for a specific phase of a workflow.
    /// Injects the Constitution, active specialist agent definitions,
    /// capability requirements, and task instruction.
    pub fn compile_phase_prompt(
        &self,
        phase: &WorkflowPhase,
        task_description: &str,
    ) -> Result<String> {
        let mut prompt = String::new();

        // 1. Ingest Constitution
        if let Ok(const_prompt) = load_constitution_prompt(&self.aios_root) {
            prompt.push_str(&const_prompt);
            prompt.push_str("\n---\n\n");
        }

        // 2. Locate active specialist agent providing capabilities for the engine
        // Let's resolve the engine (module) to its capability, and match with agent.
        let registry_dir = self.aios_root.join("registry");
        let (mod_registry, _cap_registry, _dep_registry) = parse_all_registries(&registry_dir)?;

        // Find the module that implements the engine ID
        let target_module = mod_registry.modules.values().find(|m| m.id == phase.engine);

        let mut matching_agent: Option<&AiosAgent> = None;
        if let Some(module) = target_module {
            // Find a specialist agent matching the module domain or capability
            for agent in self.agent_registry.agents.values() {
                if agent.spec.domain == module.path.trim_end_matches('/') {
                    matching_agent = Some(agent);
                    break;
                }
            }
        }

        if let Some(agent) = matching_agent {
            let agent_injection = AiosAgentRegistry::format_prompt_injection(agent);
            prompt.push_str(&agent_injection);
            prompt.push_str("\n---\n\n");
        } else {
            prompt.push_str(&format!(
                "## AIOS Module: {}\n\nYou are executing as the module engine '{}'.\n\n",
                phase.engine, phase.phase
            ));
        }

        // 3. Ingest Task details
        prompt.push_str("## Active Task Context\n\n");
        prompt.push_str(&format!("**Current Phase**: {}\n", phase.phase));
        prompt.push_str(&format!("**Engine**: {}\n", phase.engine));
        prompt.push_str(&format!("**Instruction**: {}\n\n", task_description));
        prompt.push_str("Execute your instructions matching the principles above. Do not exceed performance budgets.\n");

        Ok(prompt)
    }

    /// Execute a workflow dry-run or verification, displaying the structural phases,
    /// provider modules, specialist agents, and verification gates.
    pub fn run_workflow_diagnostics(
        &self,
        workflow_name: &str,
        task_description: &str,
    ) -> Result<()> {
        let workflow = self.load_workflow(workflow_name)?;

        println!(
            "AIOS Workflow Lifecycle Diagnostic: {} ({})",
            workflow.name, workflow.id
        );
        println!("Description: {}", workflow.description);
        println!("Triggers: {}", workflow.triggers.join(", "));
        println!("Rollback Workflow: {}", workflow.rollback_workflow);
        println!();
        println!("--------------------------------------------------");
        println!(
            "Lifecycle Execution Plan ({} Phases):",
            workflow.lifecycle.len()
        );
        println!("--------------------------------------------------");

        let registry_dir = self.aios_root.join("registry");
        let (mod_registry, _, _) = parse_all_registries(&registry_dir)?;

        for phase in &workflow.lifecycle {
            // Find module info
            let mod_name = mod_registry
                .modules
                .values()
                .find(|m| m.id == phase.engine)
                .map(|m| m.name.as_str())
                .unwrap_or("Unknown Engine");

            // Find matching agent
            let matching_agent = self.agent_registry.agents.values().find(|a| {
                mod_registry
                    .modules
                    .values()
                    .find(|m| m.id == phase.engine)
                    .map(|m| a.spec.domain == m.path.trim_end_matches('/'))
                    .unwrap_or(false)
            });

            let agent_str = matching_agent
                .map(|a| format!("{} ({})", a.spec.name, a.spec.role))
                .unwrap_or_else(|| "System Engine".to_string());

            println!(
                "  [{}] Phase: {:12} | Engine: {:20} ({:20}) | Agent: {}",
                phase.order, phase.phase, phase.engine, mod_name, agent_str
            );

            if let Some(ref gate) = phase.gate {
                println!("      ↳ Gating mechanism: {}", gate);
            }
        }
        println!("--------------------------------------------------");
        println!();

        // Display sample compiled prompt for first phase
        if let Some(first_phase) = workflow.lifecycle.first() {
            let sample_prompt = self.compile_phase_prompt(first_phase, task_description)?;
            println!("Sample Compiled Prompt for Phase [{}]:", first_phase.phase);
            println!("```markdown");
            println!("{}", sample_prompt.trim());
            println!("```");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_load_and_diagnose_real_workflow() {
        let runner = AiosWorkflowRunner::new(Path::new("../../aios")).unwrap();
        let workflow = runner.load_workflow("build-feature").unwrap();
        assert_eq!(workflow.name, "build-feature");
        assert!(!workflow.lifecycle.is_empty());

        let list = runner.list_workflows().unwrap();
        assert!(!list.is_empty());
        assert!(list.iter().any(|w| w.name == "build-feature"));

        // Compile prompt for first phase
        let first_phase = &workflow.lifecycle[0];
        let prompt = runner
            .compile_phase_prompt(first_phase, "add login button")
            .unwrap();
        assert!(prompt.contains("AIOS Constitution"));
        assert!(prompt.contains("add login button"));
    }
}
