//! `/graph` and `/codebase` slash commands — inspect and manage the Codebase Memory engine.

use crate::commands::CommandResult;
use crate::tui::app::App;
use std::fmt::Write as _;

pub fn graph(app: &mut App, arg: Option<&str>) -> CommandResult {
    codebase(app, arg)
}

pub fn codebase(app: &mut App, arg: Option<&str>) -> CommandResult {
    let sub = arg.unwrap_or("status").trim();
    let parts: Vec<&str> = sub.split_whitespace().collect();
    let action = parts.first().copied().unwrap_or("status");

    match action {
        "open" | "ui" | "browser" => {
            let port = std::env::var("HELPOFAI_GRAPH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9749);
            std::thread::spawn(move || {
                let _ = helpofai_codebase_memory::graph::GraphSupervisor::ensure_running_and_open_browser(port);
            });
            CommandResult::message(format!(
                "Opening Codebase Memory Web UI: http://localhost:{port}"
            ))
        }
        "start" => {
            let port = std::env::var("HELPOFAI_GRAPH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9749);
            std::thread::spawn(move || {
                let _ = helpofai_codebase_memory::graph::GraphSupervisor::ensure_running_and_open_browser(port);
            });
            CommandResult::message(format!("Starting Codebase Memory server on port {port}..."))
        }
        "stop" => match helpofai_codebase_memory::graph::stop_by_pid() {
            Ok(()) => CommandResult::message("Codebase Memory server stopped."),
            Err(e) => CommandResult::error(format!("Failed to stop server: {e}")),
        },
        "index" => {
            let target_path = parts
                .get(1)
                .map(|s| s.to_string())
                .unwrap_or_else(|| app.workspace.to_string_lossy().to_string());
            match helpofai_codebase_memory::manager::CodebaseMemoryManager::new() {
                Ok(manager) => match manager.index_repository(&target_path) {
                    Ok(()) => CommandResult::message(format!(
                        "Successfully indexed repository at '{target_path}'."
                    )),
                    Err(e) => CommandResult::error(format!("Indexing failed: {e}")),
                },
                Err(e) => CommandResult::error(format!(
                    "Failed to initialize Codebase Memory manager: {e}"
                )),
            }
        }
        _ => {
            let mut out = String::new();
            let _ = writeln!(out, "Codebase Memory (Knowledge Graph)");
            let _ = writeln!(out, "================================");
            let status = helpofai_codebase_memory::status::probe_status();
            let _ = writeln!(out, "Status:  {status}");
            let _ = writeln!(out);
            let _ = writeln!(out, "Commands:");
            let _ = writeln!(
                out,
                "  /graph open    - Open 3D Knowledge Graph in default browser"
            );
            let _ = writeln!(out, "  /graph start   - Start background UI server");
            let _ = writeln!(out, "  /graph stop    - Stop background UI server");
            let _ = writeln!(
                out,
                "  /graph index   - Index workspace into knowledge graph"
            );
            let _ = writeln!(out, "  /graph status  - Show current engine status");
            CommandResult::message(out)
        }
    }
}
