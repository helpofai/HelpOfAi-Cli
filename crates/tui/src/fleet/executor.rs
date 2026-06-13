//! Fleet executor — runs a fleet worker as a real `codewhale exec` subprocess.
//!
//! A fleet worker IS a headless `codewhale exec` run. There is no separate
//! "fleet worker" execution engine: the sub-agent runtime, full tool surface,
//! and recursion depth all come from the one `codewhale exec` runtime, so
//! fleet and sub-agents are one substrate (not two moving targets).
//!
//! This module is the bridge:
//! - [`build_worker_exec_command`] turns a `FleetTaskSpec` + `FleetExecConfig`
//!   into the `codewhale exec --output-format stream-json …` argv that a host
//!   adapter ([`super::host`]) launches locally or over SSH.
//! - [`map_exec_stream_line`] maps one stream-json line emitted by that worker
//!   into a [`FleetWorkerEventPayload`] for the durable ledger, so the ledger
//!   persists the worker's own event vocabulary instead of a simulated one.
//! - [`classify_worker_exit`] turns the process exit into a terminal event.
//!
//! The TUI/CLI/Runtime API observe the ledger's compact event stream — they
//! never render a child session, which is what keeps the orchestrator light at
//! high fanout.

#![allow(dead_code)]

use codewhale_config::FleetExecConfig;
use codewhale_protocol::fleet::{FleetTaskSpec, FleetWorkerEventPayload};

use super::host::FleetWorkerCommand;
use super::worker_runtime::fleet_task_prompt;

/// Build the `codewhale exec` argv that runs a fleet task headlessly.
///
/// `--auto` is always passed: a headless worker has no human to approve tool
/// calls, so it runs with full (policy-gated) tool access. `--output-format
/// stream-json` makes the worker emit the NDJSON event stream this module
/// parses. Recursion depth is inherited from the worker's own config
/// (`[runtime] max_spawn_depth`, default [`codewhale_config::DEFAULT_SPAWN_DEPTH`]).
///
/// Secrets are NEVER placed on the argv: provider credentials are resolved by
/// the worker process from its own config/keyring exactly like an interactive
/// run. The host adapter additionally refuses secret-bearing env keys.
pub fn build_worker_exec_command(
    codewhale_binary: &str,
    task_spec: &FleetTaskSpec,
    exec_config: &FleetExecConfig,
    model: Option<&str>,
) -> FleetWorkerCommand {
    let mut args: Vec<String> = vec![
        "exec".to_string(),
        "--auto".to_string(),
        "--output-format".to_string(),
        "stream-json".to_string(),
    ];

    if let Some(model) = model.map(str::trim).filter(|m| !m.is_empty()) {
        args.push("--model".to_string());
        args.push(model.to_string());
    }

    if !exec_config.allowed_tools.is_empty() {
        args.push("--allowed-tools".to_string());
        args.push(exec_config.allowed_tools.join(","));
    }
    if !exec_config.disallowed_tools.is_empty() {
        args.push("--disallowed-tools".to_string());
        args.push(exec_config.disallowed_tools.join(","));
    }
    if exec_config.max_turns > 0 && exec_config.max_turns != u32::MAX {
        args.push("--max-turns".to_string());
        args.push(exec_config.max_turns.to_string());
    }
    if !exec_config.append_system_prompt.trim().is_empty() {
        args.push("--append-system-prompt".to_string());
        args.push(exec_config.append_system_prompt.clone());
    }

    // The composed task prompt is the final positional argument.
    args.push(fleet_task_prompt(task_spec));

    FleetWorkerCommand::new(codewhale_binary.to_string(), args)
}

/// Map one `codewhale exec` stream-json line into a fleet ledger event.
///
/// Returns `None` for lines that don't correspond to a worker lifecycle
/// transition (e.g. `session_capture`, `metadata`). The exec event schema is
/// `{"type": "...", ...}` (see `ExecStreamEvent` in `main.rs`).
pub fn map_exec_stream_line(line: &str) -> Option<FleetWorkerEventPayload> {
    let value: serde_json::Value = serde_json::from_str(line.trim()).ok()?;
    match value.get("type").and_then(serde_json::Value::as_str)? {
        "tool_use" => {
            let tool = value
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("tool")
                .to_string();
            let call_id = value
                .get("id")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            Some(FleetWorkerEventPayload::RunningTool { tool, call_id })
        }
        // Streaming model output / tool results mean the worker is alive and
        // making progress; surface a coarse Running heartbeat.
        "content" | "tool_result" => Some(FleetWorkerEventPayload::Running),
        "done" => Some(FleetWorkerEventPayload::Completed {
            exit_code: Some(0),
            summary: None,
        }),
        "error" => {
            let reason = value
                .get("error")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("worker reported an error")
                .to_string();
            Some(FleetWorkerEventPayload::Failed {
                reason,
                recoverable: false,
            })
        }
        _ => None,
    }
}

/// Classify a worker process exit into a terminal fleet event.
///
/// `stopped` means the operator stopped the worker (cancellation), which takes
/// precedence over the exit code.
pub fn classify_worker_exit(exit_code: Option<i32>, stopped: bool) -> FleetWorkerEventPayload {
    if stopped {
        return FleetWorkerEventPayload::Cancelled { cancelled_by: None };
    }
    match exit_code {
        Some(0) => FleetWorkerEventPayload::Completed {
            exit_code: Some(0),
            summary: None,
        },
        Some(code) => FleetWorkerEventPayload::Failed {
            reason: format!("worker exited with code {code}"),
            recoverable: true,
        },
        None => FleetWorkerEventPayload::Failed {
            reason: "worker exited without a status code".to_string(),
            recoverable: true,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codewhale_protocol::fleet::{FleetTaskSpec, FleetTaskWorkerProfile};
    use std::collections::BTreeMap;

    fn task(instructions: &str) -> FleetTaskSpec {
        FleetTaskSpec {
            id: "t1".to_string(),
            name: "Smoke".to_string(),
            description: None,
            objective: Some("prove it runs".to_string()),
            instructions: instructions.to_string(),
            worker: Some(FleetTaskWorkerProfile {
                role: Some("reviewer".to_string()),
                tool_profile: Some("read-only".to_string()),
                tools: vec![],
                capabilities: vec![],
            }),
            workspace: None,
            input_files: vec![],
            context: vec![],
            budget: None,
            tags: vec![],
            expected_artifacts: vec![],
            scorer: None,
            retry_policy: None,
            alert_policy: None,
            timeout_seconds: None,
            metadata: BTreeMap::new(),
        }
    }

    #[test]
    fn worker_command_is_a_headless_codewhale_exec_run() {
        let exec = FleetExecConfig::default();
        let cmd = build_worker_exec_command("codewhale", &task("read the file"), &exec, None);
        assert_eq!(cmd.program, "codewhale");
        assert_eq!(cmd.args[0], "exec");
        assert!(cmd.args.contains(&"--auto".to_string()));
        // stream-json so the executor can ingest the worker's event stream.
        let joined = cmd.args.join(" ");
        assert!(joined.contains("--output-format stream-json"));
        // The task instructions ride in the positional prompt (last arg).
        assert!(cmd.args.last().unwrap().contains("read the file"));
    }

    #[test]
    fn worker_command_threads_exec_hardening_flags() {
        let exec = FleetExecConfig {
            allowed_tools: vec!["read_file".to_string(), "grep_files".to_string()],
            disallowed_tools: vec!["exec_shell".to_string()],
            max_turns: 40,
            append_system_prompt: "never push to main".to_string(),
            ..FleetExecConfig::default()
        };
        let cmd = build_worker_exec_command("codewhale", &task("audit"), &exec, Some("glm-5.1"));
        let joined = cmd.args.join(" ");
        assert!(joined.contains("--model glm-5.1"));
        assert!(joined.contains("--allowed-tools read_file,grep_files"));
        assert!(joined.contains("--disallowed-tools exec_shell"));
        assert!(joined.contains("--max-turns 40"));
        assert!(cmd.args.iter().any(|a| a == "never push to main"));
    }

    #[test]
    fn unbounded_max_turns_is_not_passed() {
        let exec = FleetExecConfig::default(); // max_turns == u32::MAX
        let cmd = build_worker_exec_command("codewhale", &task("x"), &exec, None);
        assert!(!cmd.args.join(" ").contains("--max-turns"));
    }

    #[test]
    fn stream_line_maps_tool_use_to_running_tool() {
        let line = r#"{"type":"tool_use","name":"read_file","id":"call-7","input":{}}"#;
        match map_exec_stream_line(line) {
            Some(FleetWorkerEventPayload::RunningTool { tool, call_id }) => {
                assert_eq!(tool, "read_file");
                assert_eq!(call_id.as_deref(), Some("call-7"));
            }
            other => panic!("expected RunningTool, got {other:?}"),
        }
    }

    #[test]
    fn stream_line_maps_done_and_error() {
        assert!(matches!(
            map_exec_stream_line(r#"{"type":"done"}"#),
            Some(FleetWorkerEventPayload::Completed { .. })
        ));
        match map_exec_stream_line(r#"{"type":"error","error":"boom"}"#) {
            Some(FleetWorkerEventPayload::Failed { reason, .. }) => assert_eq!(reason, "boom"),
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[test]
    fn stream_line_ignores_noise_and_bad_json() {
        assert!(map_exec_stream_line(r#"{"type":"session_capture","content":"x"}"#).is_none());
        assert!(map_exec_stream_line("not json").is_none());
        assert!(map_exec_stream_line("").is_none());
    }

    #[test]
    fn exit_classification() {
        assert!(matches!(
            classify_worker_exit(Some(0), false),
            FleetWorkerEventPayload::Completed { .. }
        ));
        assert!(matches!(
            classify_worker_exit(Some(1), false),
            FleetWorkerEventPayload::Failed {
                recoverable: true,
                ..
            }
        ));
        assert!(matches!(
            classify_worker_exit(Some(0), true),
            FleetWorkerEventPayload::Cancelled { .. }
        ));
    }
}
