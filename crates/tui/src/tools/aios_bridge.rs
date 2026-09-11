//! # AIOS Bridge Tools
//!
//! Provides two-way integration between HelpOfAi and the AIOS (AI Operating System)
//! framework:
//!
//! 1. `aios_run_and_trace`: Run shell commands with smart error filtering (~95% log compression)
//!    and symbol enrichment from the Project Brain.
//! 2. `aios_brain`: Query the Project Knowledge Graph, scan & index the workspace, check stats,
//!    or perform multi-file impact analysis for code changes.
//! 3. `aios_brain_query`: Backwards-compatible symbol search tool.
//! 4. `aios_workflow`: Full control over AIOS agentic workflows: list, inspect, diagnose, run, and history.
//! 5. `aios_trigger_workflow`: Backwards-compatible workflow triggering tool.
//! 6. `aios_registry`: Inspect modules, capabilities, dependencies, specialist agents, constitution, patterns, and templates.
//! 7. `aios_workspace`: Safe atomic file operations (read, collect, write, copy, move, delete, rollback) and upgrade analysis.
//! 8. `aios_web_inspect`: Headless browser page inspection, console/network error capture, and automatic workspace file correlation.

use async_trait::async_trait;
use serde_json::{Value, json};
use std::path::Path;
use std::process::Stdio;

use crate::tools::spec::{
    ApprovalRequirement, ToolCapability, ToolContext, ToolError, ToolResult, ToolSpec,
    optional_str, required_str,
};

// ── 1. AIOS Run and Trace Tool ───────────────────────────────────────────────

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
                    "description": "Max errors to include in the report (default: 15, max: 50). Lower = fewer tokens.",
                    "default": 15
                },
                "timeout_ms": {
                    "type": "integer",
                    "description": "Timeout in milliseconds (default: 120,000; max: 600,000).",
                    "default": 120000
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

        let default_timeout = if crate::tools::shell::is_long_running_command(command) {
            300_000
        } else {
            120_000
        };
        let timeout_ms = input
            .get("timeout_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(default_timeout)
            .clamp(1_000, 600_000);

        // ── 1. Execute command with timeout (PowerShell on Windows, sh on Unix) ──
        // AIOS tracing and diagnostics are always permitted across all modes (Agent, Plan, YOLO).

        // ── 1. Execute command with timeout ──────────────────────────────
        let (shell, shell_flag) = if cfg!(windows) {
            ("powershell", "-Command")
        } else {
            ("sh", "-c")
        };

        let mut child = tokio::process::Command::new(shell);
        child
            .arg(shell_flag)
            .arg(command)
            .current_dir(&working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let run_fut = child.output();
        let output =
            match tokio::time::timeout(std::time::Duration::from_millis(timeout_ms), run_fut).await
            {
                Ok(Ok(out)) => out,
                Ok(Err(e)) => {
                    // If powershell fails on Windows, fallback to cmd /C
                    if cfg!(windows) {
                        let mut cmd_child = tokio::process::Command::new("cmd");
                        cmd_child
                            .arg("/C")
                            .arg(command)
                            .current_dir(&working_dir)
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped());
                        match cmd_child.output().await {
                            Ok(fallback_out) => fallback_out,
                            Err(err) => {
                                return Err(ToolError::execution_failed(format!(
                                    "Failed to run command: {e}; fallback also failed: {err}"
                                )));
                            }
                        }
                    } else {
                        return Err(ToolError::execution_failed(format!(
                            "Failed to run command: {e}"
                        )));
                    }
                }
                Err(_) => {
                    return Ok(ToolResult::error(format!(
                        "Command timed out after {timeout_ms}ms"
                    )));
                }
            };

        let exit_code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let combined = format!("{stdout}\n{stderr}");

        // ── 2. Filter through AIOS error filter ──────────────────────────
        let filtered = helpofai_aios::error_filter::filter_output(&combined, exit_code);

        // ── 3. Enrich errors with Brain symbol context ────────────────────
        let brain_context = enrich_with_brain(&filtered, &context.workspace);

        // ── 4. Assemble compact AI context block ─────────────────────────
        let mut ai_report = filtered.to_ai_context();

        if !brain_context.is_empty() {
            ai_report.push_str("\n### AIOS Brain — Related Symbols\n\n");
            ai_report.push_str(&brain_context);
        }

        let success = exit_code.map(|c| c == 0).unwrap_or(false);

        Ok(ToolResult::success(ai_report).with_metadata(json!({
            "exit_code": exit_code,
            "command": command,
            "error_count": filtered.errors.len(),
            "warning_count": filtered.warning_count,
            "estimated_tokens": filtered.estimated_tokens,
            "aios_filtered": true,
            "success": success,
        })))
    }
}

// ── Brain enrichment helper ─────────────────────────────────────────────────

fn enrich_with_brain(
    filtered: &helpofai_aios::error_filter::FilteredOutput,
    workspace: &Path,
) -> String {
    if filtered.errors.is_empty() {
        return String::new();
    }

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

    let queries: Vec<String> = filtered
        .errors
        .iter()
        .take(5)
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

// ── 2. AIOS Project Brain Tool ──────────────────────────────────────────────

/// Deep code intelligence using the AIOS Project Brain knowledge graph.
pub struct AiosBrainTool;

#[async_trait]
impl ToolSpec for AiosBrainTool {
    fn name(&self) -> &'static str {
        "aios_brain"
    }

    fn description(&self) -> &'static str {
        "Interact with the AIOS Project Brain knowledge graph. \
        CRITICAL: When the user asks to understand, explore, scan, or inspect the entire project, \
        ALWAYS call this tool (action='index' or action='status') first instead of reading files directly. \
        This indexes thousands of workspace classes, functions, and symbols in milliseconds without overflowing the context window. \
        Actions: \
        'query': Search code symbols, functions, structs, classes, and signatures matching a keyword. \
        'impact': Perform multi-file impact analysis for a symbol to find callers, dependencies, and blast radius before modifying code. \
        'index': Deep scan and index workspace code into the SQLite knowledge graph. \
        'status': Check knowledge graph status, indexed file count, and symbol count."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["query", "impact", "index", "status"],
                    "description": "Brain operation to perform (default: 'query').",
                    "default": "query"
                },
                "query": {
                    "type": "string",
                    "description": "Target symbol, function, class, or keyword. Required for 'query' and 'impact'."
                },
                "token_budget": {
                    "type": "integer",
                    "description": "Max tokens to spend on query results (default: 400, max: 2000).",
                    "default": 400
                }
            },
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
        let action = optional_str(&input, "action").unwrap_or("query");

        let aios_root = match helpofai_aios::resolve_aios_root(Some(&context.workspace)) {
            Ok(root) => root,
            Err(_) => {
                return Ok(ToolResult::error(
                    "AIOS root bundle (aios.json) not found in workspace (./aios). \
                    Initialize an AIOS bundle in the workspace to enable the Project Brain.",
                ));
            }
        };

        let brain = match helpofai_aios::ProjectBrain::open(&aios_root) {
            Ok(b) => b,
            Err(e) => {
                return Ok(ToolResult::error(format!(
                    "AIOS Project Brain could not be opened: {e}. Try indexing first with action='index'."
                )));
            }
        };

        match action {
            "index" => {
                let count = brain.scan_and_index(&context.workspace).map_err(|e| {
                    ToolError::execution_failed(format!("Failed to scan and index workspace: {e}"))
                })?;
                let (files, symbols) = brain.stats().unwrap_or((count, 0));
                Ok(ToolResult::success(format!(
                    "## AIOS Project Brain Indexed\n\n\
                    - **Files Scanned & Indexed**: {count}\n\
                    - **Total Files in Graph**: {files}\n\
                    - **Total Code Symbols in Graph**: {symbols}\n\
                    - **Database**: `{}`\n\n\
                    The knowledge graph is ready for queries and impact analysis.",
                    aios_root
                        .join(".cache")
                        .join("brain")
                        .join("codebase_graph.db")
                        .display()
                ))
                .with_metadata(json!({
                    "action": "index",
                    "indexed_files": count,
                    "total_files": files,
                    "total_symbols": symbols,
                })))
            }
            "status" => {
                let (files, symbols) = brain.stats().unwrap_or((0, 0));
                let db_path = aios_root
                    .join(".cache")
                    .join("brain")
                    .join("codebase_graph.db");
                let db_size = std::fs::metadata(&db_path)
                    .map(|m| format!("{:.2} KB", m.len() as f64 / 1024.0))
                    .unwrap_or_else(|_| "Not created".to_string());

                Ok(ToolResult::success(format!(
                    "## AIOS Project Brain Status\n\n\
                    - **Status**: {}\n\
                    - **Tracked Files**: {files}\n\
                    - **Code Symbols**: {symbols}\n\
                    - **Database Path**: `{}`\n\
                    - **Database Size**: {db_size}\n",
                    if files > 0 {
                        "Active & Indexed"
                    } else {
                        "Unindexed (run action='index')"
                    },
                    db_path.display()
                ))
                .with_metadata(json!({
                    "action": "status",
                    "files": files,
                    "symbols": symbols,
                    "database": db_path.to_string_lossy(),
                })))
            }
            "impact" => {
                let query = required_str(&input, "query")?;
                let impact_md = brain.assemble_impact_markdown(query);
                if impact_md.trim().is_empty() {
                    Ok(ToolResult::success(format!(
                        "## AIOS Impact Analysis for `{query}`\n\n\
                        No callers, overrides, or direct dependants found for `{query}` in the knowledge graph.\n\
                        If the symbol was recently added or modified, run `action='index'` to refresh the graph."
                    )).with_metadata(json!({
                        "action": "impact",
                        "symbol": query,
                        "impact_found": false,
                    })))
                } else {
                    Ok(ToolResult::success(impact_md).with_metadata(json!({
                        "action": "impact",
                        "symbol": query,
                        "impact_found": true,
                    })))
                }
            }
            "query" => {
                let query = required_str(&input, "query")?;
                let budget = input
                    .get("token_budget")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.min(2000) as usize)
                    .unwrap_or(400);

                let result = brain.assemble_precision_context(query, budget * 4);

                if result.is_empty() {
                    Ok(ToolResult::success(format!(
                        "No matching code symbols found in AIOS Project Brain for `{query}`.\n\
                        Try broadening your search term or re-running with action 'index' to update the code graph."
                    ))
                    .with_metadata(json!({
                        "action": "query",
                        "query": query,
                        "found": false,
                    })))
                } else {
                    Ok(ToolResult::success(format!(
                        "## AIOS Project Brain Symbols for `{query}`\n\n{result}"
                    ))
                    .with_metadata(json!({
                        "action": "query",
                        "query": query,
                        "found": true,
                    })))
                }
            }
            other => Err(ToolError::invalid_input(format!("Unknown action: {other}"))),
        }
    }
}

// ── 3. Backwards-compatible Brain Query Tool ────────────────────────────────

/// Standalone backwards-compatible tool: AI queries symbols from the Brain.
pub struct AiosBrainQueryTool;

#[async_trait]
impl ToolSpec for AiosBrainQueryTool {
    fn name(&self) -> &'static str {
        "aios_brain_query"
    }

    fn description(&self) -> &'static str {
        "Query the AIOS Project Brain knowledge graph for code symbols, functions, \
        structs, or classes related to a query string. Returns file locations, \
        signatures, and docstrings. (Alias for aios_brain with action='query')."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Symbol name, error message, or natural language query."
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

    async fn execute(
        &self,
        mut input: Value,
        context: &ToolContext,
    ) -> Result<ToolResult, ToolError> {
        if let Some(obj) = input.as_object_mut() {
            obj.insert("action".to_string(), json!("query"));
        }
        AiosBrainTool.execute(input, context).await
    }
}

// ── 4. AIOS Workflow Tool ───────────────────────────────────────────────────

/// Full lifecycle control over AIOS agentic workflows.
pub struct AiosWorkflowTool;

#[async_trait]
impl ToolSpec for AiosWorkflowTool {
    fn name(&self) -> &'static str {
        "aios_workflow"
    }

    fn description(&self) -> &'static str {
        "Manage and execute AIOS engineering workflows. \
        Actions: \
        'list': List all available workflows (build-feature, rollback, audit-project, fix-bug, review-code, refactor, upgrade, optimize, analyze, release). \
        'inspect': View detailed phase-by-phase breakdown for a workflow. \
        'diagnose': Perform dry-run diagnostics verifying engines and specialist agents for a task. \
        'run': Execute the full multi-phase workflow lifecycle for a goal, tracking progress and logging execution journals. \
        'history': Inspect past AIOS workflow execution runs, statuses (completed/failed), and phase execution logs."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "inspect", "diagnose", "run", "history"],
                    "description": "Workflow action to perform (default: 'run').",
                    "default": "run"
                },
                "workflow_name": {
                    "type": "string",
                    "description": "Name or ID of the workflow (e.g., 'build-feature', 'review-code', 'fix-bug', 'analyze', 'audit-project', 'optimize', 'refactor', 'release', 'upgrade'). Required for inspect, diagnose, and run; optional filter for history."
                },
                "goal": {
                    "type": "string",
                    "description": "The specific objective or task description for the workflow. Required for diagnose and run."
                },
                "limit": {
                    "type": "integer",
                    "description": "Max past workflow runs to retrieve in history (default: 15).",
                    "default": 15
                }
            },
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ExecutesCode]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Suggest
    }

    fn supports_parallel(&self) -> bool {
        false
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let action = optional_str(&input, "action").unwrap_or("run");

        let aios_root = match helpofai_aios::resolve_aios_root(Some(&context.workspace)) {
            Ok(root) => root,
            Err(_) => {
                return Ok(ToolResult::error(
                    "AIOS root bundle (aios.json) not found in workspace (./aios).",
                ));
            }
        };

        let runner = helpofai_aios::AiosWorkflowRunner::new(&aios_root).map_err(|e| {
            ToolError::execution_failed(format!("Failed to initialize workflow runner: {e}"))
        })?;

        match action {
            "list" => {
                let list = runner.list_workflows().map_err(|e| {
                    ToolError::execution_failed(format!("Failed to list workflows: {e}"))
                })?;

                let mut report = format!("## Registered AIOS Workflows ({})\n\n", list.len());
                report.push_str("| Workflow | ID | Phases | Triggers | Description |\n");
                report.push_str("| --- | --- | --- | --- | --- |\n");
                for w in &list {
                    report.push_str(&format!(
                        "| **{}** | `{}` | {} | `{}` | {} |\n",
                        w.name,
                        w.id,
                        w.lifecycle.len(),
                        w.triggers.join(", "),
                        w.description
                    ));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "list",
                    "count": list.len(),
                })))
            }
            "inspect" => {
                let workflow_name = required_str(&input, "workflow_name")?;
                let workflow = runner.load_workflow(workflow_name).map_err(|e| {
                    ToolError::execution_failed(format!(
                        "Failed to load workflow '{workflow_name}': {e}"
                    ))
                })?;

                let mut report = format!(
                    "## AIOS Workflow: {} (`{}`)\n\n\
                    **Description**: {}\n\
                    **Triggers**: {}\n\n\
                    ### Lifecycle Phases ({})\n\n",
                    workflow.name,
                    workflow.id,
                    workflow.description,
                    workflow.triggers.join(", "),
                    workflow.lifecycle.len()
                );

                report.push_str("| Step | Phase | Engine Module | Gate | Output Template |\n");
                report.push_str("| --- | --- | --- | --- | --- |\n");
                for p in &workflow.lifecycle {
                    report.push_str(&format!(
                        "| {} | **{}** | `{}` | {} | {} |\n",
                        p.order,
                        p.phase,
                        p.engine,
                        p.gate.as_deref().unwrap_or("none"),
                        p.output_template.as_deref().unwrap_or("—")
                    ));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "inspect",
                    "workflow": workflow.name,
                    "id": workflow.id,
                    "phases": workflow.lifecycle.len(),
                })))
            }
            "diagnose" => {
                let workflow_name = required_str(&input, "workflow_name")?;
                let goal = required_str(&input, "goal")?;
                let workflow = runner.load_workflow(workflow_name).map_err(|e| {
                    ToolError::execution_failed(format!(
                        "Failed to load workflow '{workflow_name}': {e}"
                    ))
                })?;

                let mut phase_checks = Vec::new();
                for phase in &workflow.lifecycle {
                    let prompt = runner.compile_phase_prompt(phase, goal).map_err(|e| {
                        ToolError::execution_failed(format!("Failed to compile phase prompt: {e}"))
                    })?;
                    phase_checks.push(format!(
                        "- **Phase {} (Step {}/{})**: Engine `{}` — Prompt compiled successfully ({} bytes).",
                        phase.phase, phase.order, workflow.lifecycle.len(), phase.engine, prompt.len()
                    ));
                }

                Ok(ToolResult::success(format!(
                    "## AIOS Workflow Diagnostics: {} (`{}`)\n\n\
                    **Goal**: {}\n\
                    **Total Phases**: {}\n\n\
                    ### Phase Validations\n\n{}\n\n\
                    Workflow is valid and ready for execution.",
                    workflow.name,
                    workflow.id,
                    goal,
                    workflow.lifecycle.len(),
                    phase_checks.join("\n")
                ))
                .with_metadata(json!({
                    "action": "diagnose",
                    "workflow": workflow.name,
                    "valid": true,
                })))
            }
            "run" => {
                let workflow_name = required_str(&input, "workflow_name")?;
                let goal = required_str(&input, "goal")?;
                let workflow = runner.load_workflow(workflow_name).map_err(|e| {
                    ToolError::execution_failed(format!(
                        "Failed to load workflow '{workflow_name}': {e}"
                    ))
                })?;

                let start_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                let exe = std::env::current_exe().map_err(|e| {
                    ToolError::execution_failed(format!("Failed to locate current executable: {e}"))
                })?;

                let mut journal_phases = Vec::new();
                let mut phase_summaries = Vec::new();

                for phase in &workflow.lifecycle {
                    let prompt = runner.compile_phase_prompt(phase, goal).map_err(|e| {
                        ToolError::execution_failed(format!("Failed to compile phase prompt: {e}"))
                    })?;

                    let phase_start = std::time::Instant::now();
                    let mut cmd = tokio::process::Command::new(&exe);
                    cmd.arg("exec")
                        .arg("--auto")
                        .arg("--yolo")
                        .arg("--append-system-prompt")
                        .arg(&prompt)
                        .arg(goal)
                        .current_dir(&context.workspace)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped());

                    let output = cmd.output().await;
                    let duration = phase_start.elapsed().as_secs_f64();

                    let (success, exit_code, out_text) = match output {
                        Ok(out) => {
                            let code = out.status.code();
                            let succ = out.status.success();
                            let combined = format!(
                                "{}\n{}",
                                String::from_utf8_lossy(&out.stdout),
                                String::from_utf8_lossy(&out.stderr)
                            );
                            (succ, code, combined)
                        }
                        Err(e) => (false, None, format!("Process failed: {e}")),
                    };

                    journal_phases.push(json!({
                        "phase": phase.phase,
                        "order": phase.order,
                        "engine": phase.engine,
                        "duration_seconds": duration,
                        "success": success,
                        "exit_code": exit_code,
                    }));

                    phase_summaries.push(format!(
                        "- **Phase {} (Step {}/{})**: `{}` [Engine: `{}`] — **{}** ({:.2}s)",
                        phase.phase,
                        phase.order,
                        workflow.lifecycle.len(),
                        phase.engine,
                        phase.phase,
                        if success { "COMPLETED" } else { "FAILED" },
                        duration
                    ));

                    if !success {
                        let runs_dir = aios_root.join("runs");
                        let _ = std::fs::create_dir_all(&runs_dir);
                        let journal = json!({
                            "workflow_id": workflow.id,
                            "workflow_name": workflow.name,
                            "goal": goal,
                            "started_at": start_time,
                            "status": "failed",
                            "phases": journal_phases,
                        });
                        let run_file =
                            runs_dir.join(format!("run_{}_{start_time}.json", workflow.name));
                        if let Ok(content) = serde_json::to_string_pretty(&journal) {
                            let _ = std::fs::write(&run_file, content);
                        }

                        return Ok(ToolResult::error(format!(
                            "AIOS Workflow '{}' failed at phase '{}' after {:.2}s.\n\n{}\n\nOutput excerpt:\n```\n{}\n```",
                            workflow.name,
                            phase.phase,
                            duration,
                            phase_summaries.join("\n"),
                            out_text.chars().take(800).collect::<String>()
                        )));
                    }
                }

                // Save completed journal
                let runs_dir = aios_root.join("runs");
                let _ = std::fs::create_dir_all(&runs_dir);
                let journal = json!({
                    "workflow_id": workflow.id,
                    "workflow_name": workflow.name,
                    "goal": goal,
                    "started_at": start_time,
                    "status": "completed",
                    "phases": journal_phases,
                });
                let run_file = runs_dir.join(format!("run_{}_{start_time}.json", workflow.name));
                let _ = std::fs::write(
                    &run_file,
                    serde_json::to_string_pretty(&journal).unwrap_or_default(),
                );

                Ok(ToolResult::success(format!(
                    "## AIOS Workflow '{}' Completed Successfully\n\n\
                    **Goal**: {goal}\n\
                    **Phases Executed**: {}/{}\n\n\
                    ### Phase Breakdown\n\n{}\n\n\
                    Execution journal persisted to: `{}`",
                    workflow.name,
                    workflow.lifecycle.len(),
                    workflow.lifecycle.len(),
                    phase_summaries.join("\n"),
                    run_file.display()
                ))
                .with_metadata(json!({
                    "workflow": workflow.name,
                    "status": "completed",
                    "phases_count": workflow.lifecycle.len(),
                    "journal": run_file.to_string_lossy(),
                })))
            }
            "history" => {
                let runs_dir = aios_root.join("runs");
                if !runs_dir.exists() {
                    return Ok(ToolResult::success(
                        "No past AIOS workflow execution runs found (directory `aios/runs` does not exist yet).",
                    )
                    .with_metadata(json!({ "action": "history", "count": 0 })));
                }

                let mut entries = Vec::new();
                if let Ok(read_dir) = std::fs::read_dir(&runs_dir) {
                    for entry in read_dir.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                if let Ok(val) = serde_json::from_str::<Value>(&content) {
                                    entries.push((path, val));
                                }
                            }
                        }
                    }
                }

                if entries.is_empty() {
                    return Ok(ToolResult::success(
                        "No AIOS workflow execution logs recorded yet.",
                    )
                    .with_metadata(json!({ "action": "history", "count": 0 })));
                }

                // Sort by started_at descending
                entries.sort_by(|a, b| {
                    let a_time = a.1.get("started_at").and_then(|v| v.as_u64()).unwrap_or(0);
                    let b_time = b.1.get("started_at").and_then(|v| v.as_u64()).unwrap_or(0);
                    b_time.cmp(&a_time)
                });

                let filter_wf = optional_str(&input, "workflow_name");
                let limit = input.get("limit").and_then(|v| v.as_u64()).unwrap_or(15) as usize;

                let filtered: Vec<_> = entries
                    .into_iter()
                    .filter(|(_, val)| {
                        if let Some(target) = filter_wf {
                            let name = val
                                .get("workflow_name")
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let id = val
                                .get("workflow_id")
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            name.eq_ignore_ascii_case(target) || id.eq_ignore_ascii_case(target)
                        } else {
                            true
                        }
                    })
                    .take(limit)
                    .collect();

                if filtered.is_empty() {
                    return Ok(ToolResult::success(format!(
                        "No runs found matching workflow filter: '{}'.",
                        filter_wf.unwrap_or("")
                    )));
                }

                let mut report = format!(
                    "## AIOS Workflow Execution History ({})\n\n",
                    filtered.len()
                );
                report.push_str("| Workflow | Status | Started | Phases | Goal |\n");
                report.push_str("| --- | --- | --- | --- | --- |\n");

                let mut history_json = Vec::new();
                for (path, item) in &filtered {
                    let name = item
                        .get("workflow_name")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown");
                    let status = item
                        .get("status")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown");
                    let start_str = if let Some(s) = item.get("started_at").and_then(|s| s.as_str())
                    {
                        s.to_string()
                    } else if let Some(t) = item.get("started_at").and_then(|v| v.as_u64()) {
                        format!("{t}")
                    } else {
                        "—".to_string()
                    };
                    let phases_cnt = item
                        .get("phases")
                        .and_then(|v| v.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0);
                    let goal = item.get("goal").and_then(|s| s.as_str()).unwrap_or("—");
                    let short_goal = if goal.len() > 60 {
                        format!("{}...", &goal[..57])
                    } else {
                        goal.to_string()
                    };

                    report.push_str(&format!(
                        "| **{name}** | `{status}` | {start_str} | {phases_cnt} | {short_goal} |\n"
                    ));

                    history_json.push(json!({
                        "file": path.file_name().and_then(|s| s.to_str()).unwrap_or(""),
                        "workflow": name,
                        "status": status,
                        "started_at": start_str,
                        "phases": phases_cnt,
                        "goal": goal,
                    }));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "history",
                    "runs": history_json,
                })))
            }
            other => Err(ToolError::invalid_input(format!("Unknown action: {other}"))),
        }
    }
}

// ── 5. Backwards-compatible Workflow Trigger Tool ───────────────────────────

/// Standalone backwards-compatible tool for delegating to AIOS workflows.
pub struct AiosTriggerWorkflowTool;

#[async_trait]
impl ToolSpec for AiosTriggerWorkflowTool {
    fn name(&self) -> &'static str {
        "aios_trigger_workflow"
    }

    fn description(&self) -> &'static str {
        "Delegate a complex, multi-step task to an AIOS background workflow. \
        Use this when the user asks for a large feature, refactor, or complex task. \
        (Alias for aios_workflow with action='run')."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "workflow_name": {
                    "type": "string",
                    "description": "The name of the workflow to trigger (e.g. 'build-feature', 'review-code', 'fix-bug', 'optimize', 'refactor')."
                },
                "goal": {
                    "type": "string",
                    "description": "A detailed description of what the workflow should accomplish."
                }
            },
            "required": ["workflow_name", "goal"],
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ExecutesCode]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Suggest
    }

    fn supports_parallel(&self) -> bool {
        false
    }

    async fn execute(
        &self,
        mut input: Value,
        context: &ToolContext,
    ) -> Result<ToolResult, ToolError> {
        if let Some(obj) = input.as_object_mut() {
            obj.insert("action".to_string(), json!("run"));
        }
        AiosWorkflowTool.execute(input, context).await
    }
}

// ── 6. AIOS Registry Tool ───────────────────────────────────────────────────

/// Inspect the AIOS architecture, modules, capabilities, and specialist agents.
pub struct AiosRegistryTool;

#[async_trait]
impl ToolSpec for AiosRegistryTool {
    fn name(&self) -> &'static str {
        "aios_registry"
    }

    fn description(&self) -> &'static str {
        "Inspect the AIOS architecture, catalogued modules, registered capabilities, specialist agents, constitution, and engineering templates. \
        Actions: \
        'status': Overview of AIOS installation, modules count, capabilities count, and dependencies. \
        'list_modules': List all 28 AIOS architectural modules (ID, name, path, provider, version). \
        'list_capabilities': List all 34 capabilities, their IDs, and providing modules. \
        'list_agents': List all registered specialist agents (master, architect, backend, frontend, reviewer, qa, devops, security, database, api, etc.). \
        'get_agent': Retrieve the complete prompt, domain, and capabilities of a specific agent role. \
        'constitution': Load the 15 immutable engineering laws and the golden rule governing all AIOS code changes. \
        'patterns': List or inspect architectural and code patterns. \
        'templates': List or inspect standard engineering templates (ADR, BUG_REPORT, PLAN, ROLLBACK_PLAN, SECURITY_REPORT, etc.)."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["status", "list_modules", "list_capabilities", "list_agents", "get_agent", "constitution", "patterns", "templates"],
                    "description": "Registry inspection action (default: 'status').",
                    "default": "status"
                },
                "agent_role": {
                    "type": "string",
                    "description": "Specific agent role to inspect (e.g. 'reviewer', 'architect', 'backend', 'qa', 'devops', 'security', 'database', 'api'). Required for 'get_agent'."
                },
                "pattern_name": {
                    "type": "string",
                    "description": "Specific pattern name to view for 'patterns' action."
                },
                "template_name": {
                    "type": "string",
                    "description": "Specific template name to view for 'templates' action (e.g., 'PLAN', 'ADR_TEMPLATE', 'BUG_REPORT', 'SECURITY_REPORT', 'ROLLBACK_PLAN')."
                }
            },
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
        let action = optional_str(&input, "action").unwrap_or("status");

        let aios_root = match helpofai_aios::resolve_aios_root(Some(&context.workspace)) {
            Ok(root) => root,
            Err(_) => {
                return Ok(ToolResult::error(
                    "AIOS root bundle (aios.json) not found in workspace (./aios).",
                ));
            }
        };

        let registry_dir = aios_root.join("registry");
        let (mod_registry, cap_registry, dep_registry) =
            helpofai_aios::parse_all_registries(&registry_dir).map_err(|e| {
                ToolError::execution_failed(format!("Failed to parse AIOS registries: {e}"))
            })?;

        let agent_registry = helpofai_aios::AiosAgentRegistry::load(&aios_root).map_err(|e| {
            ToolError::execution_failed(format!("Failed to load agent registry: {e}"))
        })?;

        match action {
            "status" => Ok(ToolResult::success(format!(
                "## AIOS Architecture Registry Status\n\n\
                    - **Version**: {}\n\
                    - **Modules**: {} installed / {} total\n\
                    - **Capabilities**: {} registered\n\
                    - **Dependencies Tracked**: {}\n\
                    - **Specialist Agents**: {} registered\n\
                    - **Root Path**: `{}`\n",
                mod_registry.version,
                mod_registry.installed_count,
                mod_registry.total_count,
                cap_registry.capabilities.len(),
                dep_registry.dependencies.len(),
                agent_registry.agents.len(),
                aios_root.display()
            ))
            .with_metadata(json!({
                "action": "status",
                "installed_modules": mod_registry.installed_count,
                "total_modules": mod_registry.total_count,
                "capabilities": cap_registry.capabilities.len(),
                "agents": agent_registry.agents.len(),
            }))),
            "list_modules" => {
                let mut rows: Vec<_> = mod_registry.modules.iter().collect();
                rows.sort_by_key(|(k, _)| (*k).clone());

                let mut report = format!("## AIOS Architecture Modules ({})\n\n", rows.len());
                report.push_str("| Key | ID | Version | Path | Status |\n");
                report.push_str("| --- | --- | --- | --- | --- |\n");
                for (key, m) in rows {
                    report.push_str(&format!(
                        "| **{}** | `{}` | {} | `{}` | {} |\n",
                        key, m.id, m.version, m.path, m.status
                    ));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "list_modules",
                    "count": mod_registry.modules.len(),
                })))
            }
            "list_capabilities" => {
                let mut caps: Vec<_> = cap_registry.capabilities.iter().collect();
                caps.sort_by_key(|(name, _)| (*name).clone());

                let mut report = format!("## AIOS Capabilities ({})\n\n", caps.len());
                report.push_str("| Capability | ID | Provider Module | Module ID |\n");
                report.push_str("| --- | --- | --- | --- |\n");
                for (name, cap) in caps {
                    report.push_str(&format!(
                        "| **{}** | `{}` | {} | `{}` |\n",
                        name, cap.id, cap.provider, cap.module_id
                    ));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "list_capabilities",
                    "count": cap_registry.capabilities.len(),
                })))
            }
            "list_agents" => {
                let mut agents: Vec<_> = agent_registry.agents.values().collect();
                agents.sort_by_key(|a| a.spec.role.clone());

                let mut report = format!("## AIOS Specialist Agents ({})\n\n", agents.len());
                report.push_str(
                    "| Role | Name | ID | Domain | Thinking Budget | Required Capabilities |\n",
                );
                report.push_str("| --- | --- | --- | --- | --- | --- |\n");
                for a in agents {
                    report.push_str(&format!(
                        "| **{}** | {} | `{}` | {} | `{}` | `{}` |\n",
                        a.spec.role,
                        a.spec.name,
                        a.spec.id,
                        a.spec.domain,
                        a.spec.thinking_budget,
                        a.spec.required_capabilities.join(", ")
                    ));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "list_agents",
                    "count": agent_registry.agents.len(),
                })))
            }
            "get_agent" => {
                let role = required_str(&input, "agent_role")?;
                let agent = agent_registry.resolve(role).ok_or_else(|| {
                    ToolError::execution_failed(format!(
                        "Agent role '{role}' not found in AIOS registry."
                    ))
                })?;

                let report = format!(
                    "## AIOS Specialist Agent: {} (`{}`)\n\n\
                    - **ID**: `{}`\n\
                    - **Role**: `{}`\n\
                    - **Domain**: `{}`\n\
                    - **Thinking Budget**: `{}`\n\
                    - **Required Capabilities**: `{}`\n\
                    - **Frameworks**: `{}`\n\
                    - **Languages**: `{}`\n\
                    - **Description**: {}\n\n\
                    ### System Prompt\n\n```markdown\n{}\n```",
                    agent.spec.name,
                    agent.spec.role,
                    agent.spec.id,
                    agent.spec.role,
                    agent.spec.domain,
                    agent.spec.thinking_budget,
                    agent.spec.required_capabilities.join(", "),
                    agent.spec.frameworks.join(", "),
                    agent.spec.languages.join(", "),
                    agent.spec.description,
                    agent.prompt_body
                );

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "get_agent",
                    "role": agent.spec.role,
                    "id": agent.spec.id,
                    "domain": agent.spec.domain,
                })))
            }
            "constitution" => {
                let constitution_md = match helpofai_aios::load_constitution_prompt(&aios_root) {
                    Ok(prompt) => prompt,
                    Err(e) => {
                        let const_file = aios_root.join("constitution").join("CONSTITUTION.md");
                        if const_file.exists() {
                            std::fs::read_to_string(&const_file).unwrap_or_else(|_| e.to_string())
                        } else {
                            return Err(ToolError::execution_failed(format!(
                                "Failed to load AIOS constitution: {e}"
                            )));
                        }
                    }
                };

                Ok(ToolResult::success(constitution_md).with_metadata(json!({
                    "action": "constitution",
                })))
            }
            "patterns" => {
                let patterns_dir = aios_root.join("patterns");
                let pattern_name = optional_str(&input, "pattern_name");

                if let Some(pname) = pattern_name {
                    let target_file = if pname.ends_with(".md") || pname.ends_with(".json") {
                        patterns_dir.join(pname)
                    } else {
                        patterns_dir.join(format!("{pname}.md"))
                    };

                    if target_file.exists() {
                        let content = std::fs::read_to_string(&target_file).map_err(|e| {
                            ToolError::execution_failed(format!("Failed to read pattern file: {e}"))
                        })?;
                        return Ok(ToolResult::success(format!(
                            "## AIOS Architecture Pattern: `{pname}`\n\n{content}"
                        ))
                        .with_metadata(json!({
                            "action": "patterns",
                            "pattern": pname,
                        })));
                    } else {
                        let spec = patterns_dir.join("patterns.spec.md");
                        if spec.exists() {
                            let content = std::fs::read_to_string(&spec).unwrap_or_default();
                            return Ok(ToolResult::success(format!(
                                "## AIOS Architecture Patterns Spec\n\nPattern '{pname}' was not found as a standalone file. Here is the AIOS pattern catalog specification:\n\n{content}"
                            )));
                        }
                        return Err(ToolError::execution_failed(format!(
                            "Pattern '{pname}' not found in `{}`.",
                            patterns_dir.display()
                        )));
                    }
                }

                let mut files = Vec::new();
                if let Ok(rd) = std::fs::read_dir(&patterns_dir) {
                    for entry in rd.flatten() {
                        let p = entry.path();
                        if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                            files.push((fname.to_string(), size));
                        }
                    }
                }
                files.sort_by(|a, b| a.0.cmp(&b.0));

                let mut report = format!("## AIOS Architecture Patterns ({})\n\n", files.len());
                report.push_str("To view a specific pattern, call `aios_registry` with `action: 'patterns'` and `pattern_name: '<name>'`.\n\n");
                report.push_str("| Pattern File | Size (Bytes) |\n");
                report.push_str("| --- | --- |\n");
                for (fname, size) in &files {
                    report.push_str(&format!("| `{fname}` | {size} |\n"));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "patterns",
                    "count": files.len(),
                })))
            }
            "templates" => {
                let templates_dir = aios_root.join("templates");
                let template_name = optional_str(&input, "template_name");

                if let Some(tname) = template_name {
                    let target_file = if tname.ends_with(".md") || tname.ends_with(".json") {
                        templates_dir.join(tname)
                    } else {
                        templates_dir.join(format!("{tname}.md"))
                    };

                    if target_file.exists() {
                        let content = std::fs::read_to_string(&target_file).map_err(|e| {
                            ToolError::execution_failed(format!(
                                "Failed to read template file: {e}"
                            ))
                        })?;
                        return Ok(ToolResult::success(format!(
                            "## AIOS Template: `{tname}`\n\n```markdown\n{content}\n```"
                        ))
                        .with_metadata(json!({
                            "action": "templates",
                            "template": tname,
                        })));
                    } else {
                        return Err(ToolError::execution_failed(format!(
                            "Template '{tname}' not found in `{}`.",
                            templates_dir.display()
                        )));
                    }
                }

                let mut files = Vec::new();
                if let Ok(rd) = std::fs::read_dir(&templates_dir) {
                    for entry in rd.flatten() {
                        let p = entry.path();
                        if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                            files.push((fname.to_string(), size));
                        }
                    }
                }
                files.sort_by(|a, b| a.0.cmp(&b.0));

                let mut report = format!(
                    "## AIOS Standard Engineering Templates ({})\n\n",
                    files.len()
                );
                report.push_str("To view or copy a specific template, call `aios_registry` with `action: 'templates'` and `template_name: '<name>'`.\n\n");
                report.push_str("| Template | Size (Bytes) | Category |\n");
                report.push_str("| --- | --- | --- |\n");
                for (fname, size) in &files {
                    let cat = match fname.as_str() {
                        "ADR_TEMPLATE.md" => "Architecture Decision Record",
                        "ARCHITECTURE_REPORT.md" => "Architecture Audit",
                        "BUG_REPORT.md" => "Defect Investigation",
                        "CHANGELOG.md" => "Release & Versions",
                        "CHECKLIST.md" => "Pre-flight Verification",
                        "EXECUTION_REPORT.md" => "Workflow Journal",
                        "PERFORMANCE_REPORT.md" => "Benchmarking & Profiling",
                        "PLAN.md" => "Strategic Implementation Plan",
                        "RELEASE_NOTES.md" => "Deployment Summary",
                        "ROLLBACK_PLAN.md" => "Disaster Recovery",
                        "SECURITY_REPORT.md" => "Vulnerability Assessment",
                        _ => "General Spec",
                    };
                    report.push_str(&format!("| **`{fname}`** | {size} | {cat} |\n"));
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "templates",
                    "count": files.len(),
                })))
            }
            _ => Err(ToolError::invalid_input(format!(
                "Unknown action: {action}"
            ))),
        }
    }
}

// ── 7. AIOS Workspace Operations & Upgrade Tool ─────────────────────────────

/// High-level AIOS workspace intelligence: safely read, collect, write, copy, move,
/// delete, rollback, and modernize project files with atomic snapshots.
pub struct AiosWorkspaceTool;

#[async_trait]
impl ToolSpec for AiosWorkspaceTool {
    fn name(&self) -> &'static str {
        "aios_workspace"
    }

    fn description(&self) -> &'static str {
        "Enterprise-grade AIOS workspace operations for safely reading, collecting, writing, \
        copying, moving, deleting, and upgrading project files. All modifications are automatically \
        protected by a snapshot rollback journal.\n\n\
        CRITICAL: When the user asks to inspect, understand, or upgrade a large project, feature, or service, \
        use action='collect' or action='analyze_upgrade' instead of manually reading dozens of individual files.\n\n\
        Actions:\n\
        - 'collect': Deep multi-file collector — gathers matching files, AST symbols, callers, configs, and test suites for a target component or feature into a structured bundle.\n\
        - 'analyze_upgrade': Analyze project tech stack (Laravel/PHP, Next.js/Node, Rust, Python) and generate modernization recommendations.\n\
        - 'read': Safely read file with syntax line range and AST symbol summary.\n\
        - 'write': Atomic write with pre-image hash verification and automatic snapshot backup.\n\
        - 'copy': Safe copy of file or directory with snapshot protection.\n\
        - 'move': Atomic move/rename of file or directory with optional workspace-wide reference/import updating.\n\
        - 'delete': Safe deletion into AIOS snapshot trash (completely undoable).\n\
        - 'rollback': Rollback the most recent or specified transaction, restoring all affected files."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["collect", "read", "write", "copy", "move", "delete", "rollback", "analyze_upgrade"],
                    "description": "The workspace operation to perform."
                },
                "path": {
                    "type": "string",
                    "description": "Relative file or directory path for read, write, move, copy, or delete."
                },
                "target_path": {
                    "type": "string",
                    "description": "Destination relative path for copy or move operations."
                },
                "content": {
                    "type": "string",
                    "description": "Content to write for the 'write' action."
                },
                "query": {
                    "type": "string",
                    "description": "Search keyword, class, or symbol name for the 'collect' action."
                },
                "start_line": {
                    "type": "integer",
                    "description": "Optional starting line number for 'read'."
                },
                "end_line": {
                    "type": "integer",
                    "description": "Optional ending line number for 'read'."
                },
                "update_references": {
                    "type": "boolean",
                    "description": "For 'move': whether to update imports/references across the workspace (default: true).",
                    "default": true
                },
                "tx_id": {
                    "type": "string",
                    "description": "Optional transaction ID for 'rollback' (defaults to most recent)."
                },
                "description": {
                    "type": "string",
                    "description": "Human-readable intent or reason for the file operation (logged in transaction journal)."
                }
            },
            "required": ["action"],
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WritesFiles]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    fn supports_parallel(&self) -> bool {
        false
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let action = required_str(&input, "action")?;

        let aios_root = match helpofai_aios::resolve_aios_root(Some(&context.workspace)) {
            Ok(root) => root,
            Err(_) => {
                return Ok(ToolResult::error(
                    "AIOS root bundle (aios.json) not found in workspace (./aios). \
                    Initialize an AIOS bundle in the workspace to enable AIOS workspace operations.",
                ));
            }
        };

        let ops = helpofai_aios::AiosWorkspaceOps::new(&aios_root, &context.workspace);

        match action {
            "collect" => {
                let query = required_str(&input, "query")?;
                let bundle = ops.collect_project_context(query).map_err(|e| {
                    ToolError::execution_failed(format!("Failed to collect project context: {e}"))
                })?;

                let mut report = format!(
                    "## AIOS Project Context Bundle: `{query}`\n\n{}\n\n",
                    bundle.summary
                );

                if !bundle.matched_files.is_empty() {
                    report.push_str("### Matched Workspace Files\n\n");
                    for f in &bundle.matched_files {
                        report.push_str(&format!(
                            "* **`{}`** ({} lines, {})\n",
                            f.relative_path, f.line_count, f.language
                        ));
                        for s in &f.symbols {
                            report.push_str(&format!("    ↳ {s}\n"));
                        }
                    }
                    report.push('\n');
                }

                if !bundle.callers_and_references.is_empty() {
                    report.push_str("### Callers & Multi-File References\n\n");
                    for c in &bundle.callers_and_references {
                        report.push_str(&format!("* {c}\n"));
                    }
                    report.push('\n');
                }

                if !bundle.test_files.is_empty() {
                    report.push_str("### Associated Test Suites\n\n");
                    for t in &bundle.test_files {
                        report.push_str(&format!("* `{t}`\n"));
                    }
                    report.push('\n');
                }

                if !bundle.configs_detected.is_empty() {
                    report.push_str("### Detected Project Manifests\n\n");
                    for c in &bundle.configs_detected {
                        report.push_str(&format!("* `{c}`\n"));
                    }
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "collect",
                    "matched_files_count": bundle.matched_files.len(),
                    "symbols_count": bundle.related_symbols.len(),
                    "test_files_count": bundle.test_files.len(),
                })))
            }
            "read" => {
                let path = required_str(&input, "path")?;
                let start_line = input
                    .get("start_line")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);
                let end_line = input
                    .get("end_line")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);

                let (content, symbols) =
                    ops.read_file_rich(path, start_line, end_line)
                        .map_err(|e| {
                            ToolError::execution_failed(format!(
                                "Failed to read file '{path}': {e}"
                            ))
                        })?;

                let mut report = format!("## File: `{path}`\n\n");
                if !symbols.is_empty() {
                    report.push_str("**Declared Symbols**:\n");
                    for s in &symbols {
                        report.push_str(&format!("- {s}\n"));
                    }
                    report.push_str("\n---\n\n");
                }
                report.push_str("```\n");
                report.push_str(&content);
                report.push_str("\n```");

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "read",
                    "path": path,
                    "symbol_count": symbols.len(),
                })))
            }
            "write" => {
                let path = required_str(&input, "path")?;
                let content = required_str(&input, "content")?;
                let desc = optional_str(&input, "description").unwrap_or("AIOS file write");

                let tx_id = ops.write_file_safe(path, content, desc).map_err(|e| {
                    ToolError::execution_failed(format!("Failed to write file '{path}': {e}"))
                })?;

                Ok(ToolResult::success(format!(
                    "Successfully wrote `{path}` (atomic write with rollback snapshot saved, tx: `{tx_id}`)"
                )).with_metadata(json!({
                    "action": "write",
                    "path": path,
                    "tx_id": tx_id,
                })))
            }
            "copy" => {
                let path = required_str(&input, "path")?;
                let target = required_str(&input, "target_path")?;
                let desc = optional_str(&input, "description").unwrap_or("AIOS copy path");

                let tx_id = ops.copy_path_safe(path, target, desc).map_err(|e| {
                    ToolError::execution_failed(format!(
                        "Failed to copy '{path}' to '{target}': {e}"
                    ))
                })?;

                Ok(ToolResult::success(format!(
                    "Successfully copied `{path}` to `{target}` (tx: `{tx_id}`)"
                ))
                .with_metadata(json!({
                    "action": "copy",
                    "source": path,
                    "target": target,
                    "tx_id": tx_id,
                })))
            }
            "move" => {
                let path = required_str(&input, "path")?;
                let target = required_str(&input, "target_path")?;
                let update_refs = input
                    .get("update_references")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let desc = optional_str(&input, "description").unwrap_or("AIOS move/rename");

                let tx_id = ops
                    .move_path_safe(path, target, update_refs, desc)
                    .map_err(|e| {
                        ToolError::execution_failed(format!(
                            "Failed to move '{path}' to '{target}': {e}"
                        ))
                    })?;

                Ok(ToolResult::success(format!(
                    "Successfully moved `{path}` to `{target}` (references updated: {update_refs}, tx: `{tx_id}`)"
                )).with_metadata(json!({
                    "action": "move",
                    "source": path,
                    "target": target,
                    "tx_id": tx_id,
                })))
            }
            "delete" => {
                let path = required_str(&input, "path")?;
                let desc = optional_str(&input, "description").unwrap_or("AIOS delete path");

                let tx_id = ops.delete_path_safe(path, desc).map_err(|e| {
                    ToolError::execution_failed(format!("Failed to delete '{path}': {e}"))
                })?;

                Ok(ToolResult::success(format!(
                    "Successfully deleted `{path}` (snapshot archived in AIOS trash, tx: `{tx_id}`)"
                ))
                .with_metadata(json!({
                    "action": "delete",
                    "path": path,
                    "tx_id": tx_id,
                })))
            }
            "rollback" => {
                let tx_id = optional_str(&input, "tx_id");
                let result_msg = ops.rollback_transaction(tx_id).map_err(|e| {
                    ToolError::execution_failed(format!("Failed to rollback transaction: {e}"))
                })?;

                Ok(ToolResult::success(result_msg).with_metadata(json!({
                    "action": "rollback",
                    "tx_id": tx_id,
                })))
            }
            "analyze_upgrade" => {
                let analysis = ops.analyze_upgrade().map_err(|e| {
                    ToolError::execution_failed(format!("Failed to analyze project upgrade: {e}"))
                })?;

                let mut report = format!(
                    "## AIOS Project Modernization & Upgrade Analysis\n\n\
                    - **Project Type**: {}\n\
                    - **Detected Frameworks**: {}\n\
                    - **Suggested AIOS Workflow**: `{}`\n\n\
                    ### Recommendations\n\n",
                    analysis.project_type,
                    analysis.detected_frameworks.join(", "),
                    analysis.suggested_workflow
                );

                for r in &analysis.recommendations {
                    report.push_str(&format!(
                        "#### [{}] {}\n\
                        - **Current Version**: {}\n\
                        - **Target Version**: {}\n\
                        - **Details**: {}\n\
                        - **Migration Steps**:\n",
                        r.severity,
                        r.component,
                        r.current_version.as_deref().unwrap_or("N/A"),
                        r.target_version.as_deref().unwrap_or("Latest"),
                        r.details
                    ));
                    for s in &r.migration_steps {
                        report.push_str(&format!("  1. {s}\n"));
                    }
                    report.push('\n');
                }

                if !analysis.breaking_changes_warning.is_empty() {
                    report.push_str("### Potential Breaking Changes / Caveats\n\n");
                    for w in &analysis.breaking_changes_warning {
                        report.push_str(&format!("* ⚠ {w}\n"));
                    }
                }

                Ok(ToolResult::success(report).with_metadata(json!({
                    "action": "analyze_upgrade",
                    "project_type": analysis.project_type,
                    "recommendations_count": analysis.recommendations.len(),
                })))
            }
            _ => Err(ToolError::invalid_input(format!(
                "Unknown action: {action}"
            ))),
        }
    }
}

// ── 8. AIOS Web Inspector Tool ──────────────────────────────────────────────

/// Headless web inspection and browser error diagnostics for local and remote websites.
pub struct AiosWebInspectTool;

#[async_trait]
impl ToolSpec for AiosWebInspectTool {
    fn name(&self) -> &'static str {
        "aios_web_inspect"
    }

    fn description(&self) -> &'static str {
        "Inspect a local web application (e.g., http://localhost:3000, http://127.0.0.1:8080, http://192.168.x.x) \
        or remote website using a headless browser. Captures rendered page title/text, browser console errors \
        (console.error, console.warn), JavaScript runtime exceptions, and failed HTTP requests (4xx/5xx, CORS). \
        Automatically correlates errors with local workspace source files (.js, .jsx, .ts, .tsx, .vue, .svelte, .html) \
        and extracts code snippets with suggested fixes for rapid auto-patching."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to inspect (e.g. 'http://localhost:3000', 'http://127.0.0.1:5173', 'http://192.168.1.50:8080', or public URL)."
                },
                "wait_ms": {
                    "type": "integer",
                    "description": "Milliseconds to wait after page load for client-side JavaScript hydration and async requests (default: 1500).",
                    "default": 1500
                },
                "engine": {
                    "type": "string",
                    "enum": ["auto", "cdp", "playwright", "puppeteer", "http"],
                    "description": "Browser inspection engine: 'auto' (recommended), 'cdp' (native Chrome/Edge DevTools), 'playwright', 'puppeteer', or 'http' (lightweight HTTP probe).",
                    "default": "auto"
                },
                "correlate_workspace": {
                    "type": "boolean",
                    "description": "Whether to correlate browser errors with local workspace source files (default: true).",
                    "default": true
                }
            },
            "required": ["url"],
            "additionalProperties": false
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::Network, ToolCapability::ReadOnly]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    fn supports_parallel(&self) -> bool {
        false
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let url = required_str(&input, "url")?;
        let wait_ms = input
            .get("wait_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(1500);
        let engine_str = optional_str(&input, "engine").unwrap_or("auto");
        let correlate_workspace = input
            .get("correlate_workspace")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let options = helpofai_aios::WebInspectOptions {
            url: url.to_string(),
            wait_ms,
            engine: helpofai_aios::WebInspectEngine::from(engine_str),
            correlate_workspace,
        };

        let workspace = context.workspace.clone();

        let report = tokio::task::spawn_blocking(move || {
            helpofai_aios::inspect_web_page(&options, Some(&workspace))
        })
        .await
        .map_err(|e| ToolError::execution_failed(format!("Task spawn failed: {e}")))?
        .map_err(|e| ToolError::execution_failed(format!("Web inspection failed: {e}")))?;

        Ok(ToolResult::success(report.summary.clone()).with_metadata(json!(report)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn aios_registry_tool_status_and_listing() {
        let tool = AiosRegistryTool;
        let ws = std::env::current_dir().unwrap();
        let ctx = ToolContext::new(ws);

        // Status
        let res = tool
            .execute(json!({ "action": "status" }), &ctx)
            .await
            .unwrap();
        assert!(res.success);
        assert!(res.content.contains("Modules"));
        assert!(res.content.contains("Capabilities"));

        // List Workflows via AiosWorkflowTool
        let wf_tool = AiosWorkflowTool;
        let wf_res = wf_tool
            .execute(json!({ "action": "list" }), &ctx)
            .await
            .unwrap();
        assert!(wf_res.success);
        assert!(wf_res.content.contains("build-feature"));
        assert!(wf_res.content.contains("review-code"));

        // Inspect workflow
        let inspect_res = wf_tool
            .execute(
                json!({
                    "action": "inspect",
                    "workflow_name": "build-feature"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(inspect_res.success);
        assert!(inspect_res.content.contains("spec"));
        assert!(inspect_res.content.contains("code"));

        // List Agents
        let agent_res = tool
            .execute(json!({ "action": "list_agents" }), &ctx)
            .await
            .unwrap();
        assert!(agent_res.success);
        assert!(agent_res.content.contains("reviewer"));
        assert!(agent_res.content.contains("architect"));

        // Get Agent
        let get_res = tool
            .execute(
                json!({
                    "action": "get_agent",
                    "agent_role": "reviewer"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(get_res.success);
        assert!(get_res.content.contains("AIOS-AGENT-000016"));

        // Brain Tool Status
        let brain_tool = AiosBrainTool;
        let brain_status_res = brain_tool
            .execute(json!({ "action": "status" }), &ctx)
            .await
            .unwrap();
        assert!(brain_status_res.success);
        assert!(brain_status_res.content.contains("AIOS Project Brain"));

        // Brain Tool Query
        let brain_query_res = brain_tool
            .execute(
                json!({
                    "action": "query",
                    "query": "AiosWorkflowTool"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(brain_query_res.success);

        // Workspace Tool Test
        let ws_tool = AiosWorkspaceTool;
        let analyze_res = ws_tool
            .execute(json!({ "action": "analyze_upgrade" }), &ctx)
            .await
            .unwrap();
        assert!(analyze_res.success);
        assert!(analyze_res.content.contains("Project Modernization"));

        let write_res = ws_tool
            .execute(
                json!({
                    "action": "write",
                    "path": "test_aios_file.txt",
                    "content": "hello aios workspace"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(write_res.success);
        assert!(write_res.content.contains("Successfully wrote"));

        let read_res = ws_tool
            .execute(
                json!({
                    "action": "read",
                    "path": "test_aios_file.txt"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(read_res.success);
        assert!(read_res.content.contains("hello aios workspace"));

        let del_res = ws_tool
            .execute(
                json!({
                    "action": "delete",
                    "path": "test_aios_file.txt"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(del_res.success);
        assert!(del_res.content.contains("Successfully deleted"));

        // Workflow history
        let hist_res = wf_tool
            .execute(json!({ "action": "history" }), &ctx)
            .await
            .unwrap();
        assert!(hist_res.success);

        // Registry constitution
        let const_res = tool
            .execute(json!({ "action": "constitution" }), &ctx)
            .await
            .unwrap();
        assert!(const_res.success);
        assert!(const_res.content.contains("AIOS Constitution"));

        // Registry patterns
        let patterns_res = tool
            .execute(json!({ "action": "patterns" }), &ctx)
            .await
            .unwrap();
        assert!(patterns_res.success);
        assert!(patterns_res.content.contains("Patterns"));

        // Registry templates
        let templates_res = tool
            .execute(json!({ "action": "templates" }), &ctx)
            .await
            .unwrap();
        assert!(templates_res.success);
        assert!(templates_res.content.contains("Templates"));

        let plan_tpl_res = tool
            .execute(
                json!({
                    "action": "templates",
                    "template_name": "PLAN.md"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(plan_tpl_res.success);
        assert!(plan_tpl_res.content.contains("PLAN.md"));

        // AiosWebInspectTool Spec
        let web_inspect_tool = AiosWebInspectTool;
        assert_eq!(web_inspect_tool.name(), "aios_web_inspect");
        assert_eq!(
            web_inspect_tool.approval_requirement(),
            ApprovalRequirement::Auto
        );
        assert!(
            web_inspect_tool
                .capabilities()
                .contains(&ToolCapability::Network)
        );
    }
}
