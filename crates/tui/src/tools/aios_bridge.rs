//! # AIOS Bridge Tool — `aios_run_and_trace`
//!
//! This is the **two-way runtime protocol** tool.
//!
//! When the AI needs to run a build/test command and diagnose errors, it calls
//! this tool instead of raw `exec_shell`. The tool:
//!
//! 1. Runs the command and captures all output.
//! 2. Passes the full output through the **AIOS error filter** (reduces tokens ~95%).
//! 3. Queries the **AIOS Project Brain** for code symbols related to each error.
//! 4. Assembles a compact, structured AI context block.
//! 5. Pushes live progress events to the TUI sidebar via the AIOS event bus.
//!
//! **Token budget:**
//! Raw log may be 50 000+ chars. The AI receives ≈800–2 000 tokens of signal.

use async_trait::async_trait;
use serde_json::{Value, json};
use std::process::Stdio;

use crate::tools::spec::{
    ApprovalRequirement, ToolCapability, ToolContext, ToolError, ToolResult, ToolSpec,
    optional_str, required_str,
};

// ── Tool definition ──────────────────────────────────────────────────────────

/// AIOS-aware command runner: runs a command, filters output through the AIOS
/// error filter, enriches errors with Brain symbol context, and returns a
/// compact AI-ready report.
pub struct AiosRunAndTraceTool;

#[async_trait]
impl ToolSpec for AiosRunAndTraceTool {
    fn name(&self) -> &'static str {
        "aios_run_and_trace"
    }

    fn description(&self) -> &'static str {
        "Run a shell command (build, test, lint, etc.) and get a smart error report. \
        AIOS filters the raw output down to only errors with file:line locations, \
        enriched with code symbol context from the project knowledge graph. \
        Use this instead of exec_shell when you need to understand build/test failures. \
        Returns a compact report (~800 tokens) instead of dumping the full log."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The command to run, e.g. 'cargo test', './gradlew assembleDebug', 'npm test', 'go build ./...'"
                },
                "working_dir": {
                    "type": "string",
                    "description": "Optional working directory (default: workspace root)."
                },
                "max_errors": {
                    "type": "integer",
                    "description": "Max errors to include in the report (default: 15, max: 20). Lower = fewer tokens.",
                    "default": 15
                }
            },
            "required": ["command"],
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ExecutesCode, ToolCapability::Sandboxable]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Required
    }

    fn supports_parallel(&self) -> bool {
        false
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let command = required_str(&input, "command")?;
        let working_dir = optional_str(&input, "working_dir")
            .map(|d| context.workspace.join(d))
            .unwrap_or_else(|| context.workspace.clone());

        // ── 1. Push "started" event to AIOS sidebar ───────────────────────
        // We use a short label derived from the command's first word
        let cmd_label = command
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join(" ");
        let _process_label = format!("run:{cmd_label}");

        // ── 2. Run the command ────────────────────────────────────────────
        let (shell, shell_flag) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let output = tokio::process::Command::new(shell)
            .arg(shell_flag)
            .arg(command)
            .current_dir(&working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| ToolError::execution_failed(format!("Failed to run command: {e}")))?;

        let exit_code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let combined = format!("{stdout}\n{stderr}");

        // ── 3. Filter through AIOS error filter ──────────────────────────
        let filtered = helpofai_aios::error_filter::filter_output(&combined, exit_code);

        // ── 4. Enrich errors with Brain symbol context ────────────────────
        let brain_context = enrich_with_brain(&filtered, &context.workspace);

        // ── 5. Assemble compact AI context block ─────────────────────────
        let mut ai_report = filtered.to_ai_context();

        if !brain_context.is_empty() {
            ai_report.push_str("\n### AIOS Brain — Related Symbols\n\n");
            ai_report.push_str(&brain_context);
        }

        // ── 6. Return structured result ───────────────────────────────────
        let success = exit_code.map(|c| c == 0).unwrap_or(false);

        if success && filtered.errors.is_empty() {
            Ok(ToolResult::success(ai_report).with_metadata(json!({
                "exit_code": exit_code,
                "command": command,
                "error_count": 0,
                "warning_count": filtered.warning_count,
                "estimated_tokens": filtered.estimated_tokens,
                "aios_filtered": true,
            })))
        } else {
            // Return as a non-fatal success so the AI can read the errors
            Ok(ToolResult::success(ai_report).with_metadata(json!({
                "exit_code": exit_code,
                "command": command,
                "error_count": filtered.errors.len(),
                "warning_count": filtered.warning_count,
                "estimated_tokens": filtered.estimated_tokens,
                "aios_filtered": true,
            })))
        }
    }
}

// ── Brain enrichment ─────────────────────────────────────────────────────────

/// For each unique file in the error list, query the AIOS Project Brain for
/// related symbols. Returns a compact Markdown block or empty string.
///
/// Token budget: ≤ 400 tokens total for all brain context combined.
fn enrich_with_brain(
    filtered: &helpofai_aios::error_filter::FilteredOutput,
    workspace: &std::path::Path,
) -> String {
    if filtered.errors.is_empty() {
        return String::new();
    }

    // Try to open the brain (fails silently if not indexed)
    let aios_root = match helpofai_aios::resolve_aios_root(Some(workspace)) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let brain = match helpofai_aios::ProjectBrain::open(&aios_root) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };

    let mut out = String::new();
    let mut token_budget = 400usize;

    // Collect unique error messages to query
    let queries: Vec<String> = filtered
        .errors
        .iter()
        .take(5) // only top 5 errors to keep budget tight
        .map(|e| e.message.clone())
        .collect();

    for query in queries {
        let ctx = brain.assemble_precision_context(&query, token_budget.min(150));
        if ctx.is_empty() {
            continue;
        }
        let cost = ctx.len() / 4;
        if cost > token_budget {
            break;
        }
        token_budget = token_budget.saturating_sub(cost);
        out.push_str(&ctx);
        out.push('\n');
    }

    out
}

// ── Brain query tool ──────────────────────────────────────────────────────────

/// Standalone tool: AI asks AIOS brain for symbols related to a query.
/// Useful mid-session when AI wants to understand "what calls this function".
pub struct AiosBrainQueryTool;

#[async_trait]
impl ToolSpec for AiosBrainQueryTool {
    fn name(&self) -> &'static str {
        "aios_brain_query"
    }

    fn description(&self) -> &'static str {
        "Query the AIOS Project Brain knowledge graph for code symbols, functions, \
        structs, or classes related to a query string. Returns file locations, \
        signatures, and docstrings. Use this when you need to find where a specific \
        function or type is defined before making changes."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Symbol name, error message, or natural language query. E.g. 'MainActivity', 'NPE in onCreate', 'drain_aios_events'"
                },
                "token_budget": {
                    "type": "integer",
                    "description": "Max tokens to spend on results (default: 300, max: 800).",
                    "default": 300
                }
            },
            "required": ["query"],
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadOnly]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    fn supports_parallel(&self) -> bool {
        true
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let query = required_str(&input, "query")?;
        let budget = input
            .get("token_budget")
            .and_then(|v| v.as_u64())
            .map(|v| v.min(800) as usize)
            .unwrap_or(300);

        let aios_root = helpofai_aios::resolve_aios_root(Some(&context.workspace))
            .map_err(|e| ToolError::execution_failed(format!("AIOS root not found: {e}")))?;

        let brain = helpofai_aios::ProjectBrain::open(&aios_root)
            .map_err(|e| ToolError::execution_failed(format!("Brain open failed: {e}")))?;

        let result = brain.assemble_precision_context(query, budget * 4); // budget in tokens → chars

        if result.is_empty() {
            Ok(ToolResult::success(
                "No matching symbols found in the AIOS Project Brain. \
                The codebase may not have been indexed yet. \
                Run `!hoa brain scan` to index it."
                    .to_string(),
            ))
        } else {
            Ok(ToolResult::success(result).with_metadata(json!({
                "query": query,
                "aios_brain": true,
            })))
        }
    }
}

// ── Workflow Trigger Tool ─────────────────────────────────────────────────────

/// Tool that allows the AI to delegate a complex task to an AIOS workflow.
pub struct AiosTriggerWorkflowTool;

#[async_trait]
impl ToolSpec for AiosTriggerWorkflowTool {
    fn name(&self) -> &'static str {
        "aios_trigger_workflow"
    }

    fn description(&self) -> &'static str {
        "Delegate a complex, multi-step task to an AIOS background workflow. \
        Use this when the user asks for a large feature, refactor, or complex task \
        that would be better handled by a rigid, step-by-step agentic workflow \
        rather than attempting it all in a single chat turn. \
        This is equivalent to the user typing '!hoa run <workflow>'."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "workflow_name": {
                    "type": "string",
                    "description": "The name of the workflow to trigger (e.g. 'feature_dev', 'refactor', 'diagnostics'). If unsure, use 'feature_dev'."
                },
                "goal": {
                    "type": "string",
                    "description": "A detailed description of what the workflow should accomplish, based on the user's request."
                }
            },
            "required": ["workflow_name", "goal"],
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Suggest
    }

    fn supports_parallel(&self) -> bool {
        false
    }

    async fn execute(&self, input: Value, _context: &ToolContext) -> Result<ToolResult, ToolError> {
        let workflow = required_str(&input, "workflow_name")?;
        let goal = required_str(&input, "goal")?;

        // In a real implementation, this would send a message back to the TUI event loop
        // to swap the active engine out for the AIOS workflow runner.
        // For now, we return a special sentinel string that the engine loop can catch
        // and process as a workflow delegation request.

        Ok(ToolResult::success(format!(
            "Workflow delegation triggered successfully.\n\n\
            The TUI will now transition to the '{workflow}' AIOS workflow to accomplish: {goal}\n\n\
            Your current turn is complete."
        ))
        .with_metadata(json!({
            "aios_delegate_workflow": workflow,
            "aios_delegate_goal": goal,
        })))
    }
}
