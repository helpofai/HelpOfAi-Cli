//! # helpofai-aios — Types
//!
//! Serde‑backed Rust types for every AIOS 1.0 JSON contract.
//! Mirrors `AIOS-Skills-V01-0/aios/schemas/*.json` faithfully.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Identifiers ──────────────────────────────────────────────────

/// A strong-typed module id (`AIOS-MODULE-000002`, `AIOS-CAPABILITY-000010`, etc.).
pub type ModuleId = String;

/// Convenience constructor for a ModuleId.
#[inline]
pub fn module_id(s: &str) -> ModuleId {
    s.to_string()
}

// ── Root Manifest (aios.json) ──────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AiOSRoot {
    pub id: ModuleId,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub codename: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub constitution_version: String,
    #[serde(default)]
    pub schema_version: String,
    #[serde(default)]
    pub compatibility: Compatibility,
    #[serde(default)]
    pub architecture: AiOSArchitecture,
    #[serde(default)]
    pub subsystems: HashMap<String, SubsystemEntry>,
    #[serde(default)]
    pub design_principle: String,
    #[serde(default)]
    pub golden_rule: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Compatibility {
    #[serde(default)]
    pub cli_min_version: String,
    #[serde(default)]
    pub aios_api_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AiOSArchitecture {
    #[serde(default)]
    pub principle: String,
    #[serde(default)]
    pub loading_model: String,
    #[serde(default)]
    pub core_layers: Vec<CoreLayer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CoreLayer {
    #[serde(default)]
    pub layer: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubsystemEntry {
    pub id: ModuleId,
    pub path: String,
    pub load_order: i64,
    pub required: bool,
}

// ── Module Manifest (module.json) ───────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModuleManifest {
    /// JSON schema `$schema` pointer.
    #[serde(rename = "$schema", default)]
    pub schema: String,
    pub id: ModuleId,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub manifest_version: String,
    #[serde(default)]
    pub codename: String,
    #[serde(default, alias = "type")]
    pub module_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub load_order: i64,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub optional_depends_on: Vec<String>,
    #[serde(default)]
    pub conflicts_with: Vec<String>,
    #[serde(default)]
    pub provides: Vec<CapabilityDef>,
    #[serde(default)]
    pub entry_point: String,
    #[serde(default)]
    pub permissions: Vec<PermissionDef>,
    #[serde(default)]
    pub performance_budget: Option<PerformanceBudget>,
    #[serde(default)]
    pub maturity: String,
    #[serde(default)]
    pub architecture_patterns: Vec<String>,
    #[serde(default)]
    pub compatibility: ModuleCompatibility,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub contracts: Vec<String>,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CapabilityDef {
    pub id: ModuleId,
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionDef {
    pub scope: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub trust_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PerformanceBudget {
    #[serde(default)]
    pub max_load_ms: i64,
    #[serde(default)]
    pub max_memory_mb: i64,
    #[serde(default)]
    pub max_cache_mb: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ModuleCompatibility {
    #[serde(default)]
    pub aios_min_version: String,
    #[serde(default)]
    pub aios_max_version: String,
    #[serde(default)]
    pub constitution_target: String,
}

// ── Agent Definitions (AGENT-*.json) ──────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AgentDef {
    #[serde(rename = "$schema", default)]
    pub schema: String,
    pub id: ModuleId,
    pub name: String,
    pub role: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub frameworks: Vec<String>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub required_capabilities: Vec<String>,
    #[serde(default)]
    pub sub_agents: Vec<String>,
    #[serde(default)]
    pub thinking_budget: String,
}

// ── Workflow Definition (WORKFLOW-*.json) ─────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowDef {
    #[serde(rename = "$schema", default)]
    pub schema: String,
    pub id: ModuleId,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub triggers: Vec<String>,
    #[serde(default)]
    pub lifecycle: Vec<WorkflowPhase>,
    #[serde(default)]
    pub rollback_workflow: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowPhase {
    pub phase: String,
    pub order: i64,
    pub engine: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub gate: Option<String>,
    #[serde(default)]
    pub output_template: Option<String>,
}

// ── Contract Definition (contract.json) ────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractDef {
    #[serde(rename = "$schema", default)]
    pub schema: String,
    #[serde(default)]
    pub id: ModuleId,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub inputs: Vec<ContractParam>,
    #[serde(default)]
    pub outputs: Vec<ContractParam>,
    #[serde(default)]
    pub side_effects: Vec<String>,
    #[serde(default)]
    pub error_codes: Vec<ErrorCode>,
    #[serde(default)]
    pub performance_guarantees: Option<PerformanceBudget>,
    #[serde(default)]
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ContractParam {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub param_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub validation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorCode {
    pub code: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub recovery: String,
}

// ── Constitution (constitution.json) ──────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", default)]
pub struct Constitution {
    #[serde(default)]
    pub id: ModuleId,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "type", default)]
    pub constitution_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub principles: Vec<Principle>,
    #[serde(default)]
    pub golden_rule: Option<GoldenRule>,
    #[serde(default)]
    pub supremacy_order: Vec<String>,
    #[serde(default)]
    pub compatibility: Compatibility,
    #[serde(default)]
    pub updated: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Principle {
    pub id: ModuleId,
    pub name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub enforcement: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GoldenRule {
    #[serde(default)]
    pub id: ModuleId,
    #[serde(default)]
    pub statement: String,
    #[serde(default)]
    pub priority: String,
}

// ── Integration Contract (integration.json) ────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrationContract {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub target_cli_version: String,
    #[serde(default)]
    pub api_version: String,
    #[serde(default)]
    pub entry_points: HashMap<String, String>,
}

// ── Capability Spec ────────────────────────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capability {
    pub id: ModuleId,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub module_id: ModuleId,
}

// ── Dependency Entry ──────────────────────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DependencyEntry {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub installed_at: String,
    #[serde(default)]
    pub optional: bool,
}

// ── Module Catalogue Entry ────────────────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModuleCatalogEntry {
    pub id: ModuleId,
    pub name: String,
    pub version: String,
    pub path: String,
    #[serde(default)]
    pub status: String,
}

// ── Schema Info ─────────────────────────────────────────────────
// Reflects top-level schema read for tool validation

#[derive(Debug, Clone, Default)]
pub struct SchemaInfo {
    pub id: ModuleId,
    pub title: String,
    pub description: String,
    pub required_fields: Vec<String>,
    pub properties_count: usize,
}

// ── Agent Capability Matrix (agent-capability-matrix.md) ──────

#[derive(Debug, Clone, Default)]
pub struct AgentCapabilityMatrix {
    pub agent_id: ModuleId,
    pub capabilities: Vec<String>,
}
