//! Session command area: saving, forking, resuming, exporting, and the
//! `/relay` session-handoff artifact.

mod rename;
// This group dir intentionally has a `session.rs` child module with the same
// name. The module_inception allow is a permanent structure rationale, not
// migration scaffolding; see docs/architecture/command-dispatch.md.
#[allow(clippy::module_inception)]
mod session;

use std::fmt::Write as _;

use crate::commands::CommandResult;
use crate::commands::traits::{Command, CommandGroup, CommandInfo, FunctionCommand};
use crate::localization::MessageId;
use crate::tui::app::{App, AppAction};

pub struct SessionCommands;

impl CommandGroup for SessionCommands {
    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(FunctionCommand::new(&RENAME_INFO, run_rename)),
            Box::new(FunctionCommand::new(&SAVE_INFO, run_save)),
            Box::new(FunctionCommand::new(&FORK_INFO, run_fork)),
            Box::new(FunctionCommand::new(&NEW_INFO, run_new)),
            Box::new(FunctionCommand::new(&SESSIONS_INFO, run_sessions)),
            Box::new(FunctionCommand::new(&LOAD_INFO, run_load)),
            Box::new(FunctionCommand::new(&COMPACT_INFO, run_compact)),
            Box::new(FunctionCommand::new(&PURGE_INFO, run_purge)),
            Box::new(FunctionCommand::new(&RELAY_INFO, run_relay)),
            Box::new(FunctionCommand::new(&EXPORT_INFO, run_export)),
        ]
    }
}

static RENAME_INFO: CommandInfo = CommandInfo {
    name: "rename",
    aliases: &["gaiming", "chongmingming"],
    usage: "/rename <new title>",
    description_id: MessageId::CmdRenameDescription,
};
static SAVE_INFO: CommandInfo = CommandInfo {
    name: "save",
    aliases: &[],
    usage: "/save [path]",
    description_id: MessageId::CmdSaveDescription,
};
static FORK_INFO: CommandInfo = CommandInfo {
    name: "fork",
    aliases: &["branch"],
    usage: "/fork",
    description_id: MessageId::CmdForkDescription,
};
static NEW_INFO: CommandInfo = CommandInfo {
    name: "new",
    aliases: &[],
    usage: "/new [--force]",
    description_id: MessageId::CmdNewDescription,
};
static SESSIONS_INFO: CommandInfo = CommandInfo {
    name: "sessions",
    aliases: &["resume"],
    usage: "/sessions [show|prune <days>]",
    description_id: MessageId::CmdSessionsDescription,
};
static LOAD_INFO: CommandInfo = CommandInfo {
    name: "load",
    aliases: &["jiazai"],
    usage: "/load [path]",
    description_id: MessageId::CmdLoadDescription,
};
static COMPACT_INFO: CommandInfo = CommandInfo {
    name: "compact",
    aliases: &["yasuo"],
    usage: "/compact",
    description_id: MessageId::CmdCompactDescription,
};
static PURGE_INFO: CommandInfo = CommandInfo {
    name: "purge",
    aliases: &["qingchu"],
    usage: "/purge",
    description_id: MessageId::CmdPurgeDescription,
};
static RELAY_INFO: CommandInfo = CommandInfo {
    name: "relay",
    aliases: &["batonpass", "接力"],
    usage: "/relay [focus]",
    description_id: MessageId::CmdRelayDescription,
};
static EXPORT_INFO: CommandInfo = CommandInfo {
    name: "export",
    aliases: &["daochu"],
    usage: "/export [path]",
    description_id: MessageId::CmdExportDescription,
};

fn run_registered(app: &mut App, name: &str, arg: Option<&str>) -> CommandResult {
    dispatch(app, name, arg).expect("registered session command should dispatch")
}

fn run_rename(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "rename", arg)
}
fn run_save(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "save", arg)
}
fn run_fork(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "fork", arg)
}
fn run_new(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "new", arg)
}
fn run_sessions(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "sessions", arg)
}
fn run_load(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "load", arg)
}
fn run_compact(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "compact", arg)
}
fn run_purge(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "purge", arg)
}
fn run_relay(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "relay", arg)
}
fn run_export(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "export", arg)
}

pub(in crate::commands) fn dispatch(
    app: &mut App,
    command: &str,
    arg: Option<&str>,
) -> Option<CommandResult> {
    let result = match command {
        "rename" | "gaiming" | "chongmingming" => rename::rename(app, arg),
        "save" => session::save(app, arg),
        "fork" | "branch" => session::fork(app),
        "new" => session::new_session(app, arg),
        "sessions" | "resume" => session::sessions(app, arg),
        "relay" | "batonpass" | "接力" => relay(app, arg),
        "load" | "jiazai" => session::load(app, arg),
        "compact" | "yasuo" => session::compact(app),
        "purge" | "qingchu" => session::purge(app),
        "export" | "daochu" => session::export(app, arg),
        _ => return None,
    };
    Some(result)
}

/// Ask the active model to write a compact relay artifact for the next thread.
///
/// The visible command is `/relay` (with `/接力` for Chinese users), but the
/// durable file path remains `.deepseek/handoff.md` for compatibility with
/// existing sessions and startup prompt loading.
pub fn relay(app: &mut App, arg: Option<&str>) -> CommandResult {
    let focus = arg.map(str::trim).filter(|value| !value.is_empty());
    let message = build_relay_instruction(app, focus);
    CommandResult::with_message_and_action(
        "Preparing session relay at .deepseek/handoff.md...",
        AppAction::SendMessage(message),
    )
}

fn build_relay_instruction(app: &App, focus: Option<&str>) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Create a compact session relay (接力) for a future HelpOfAi thread."
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "Write or update `.deepseek/handoff.md`.");
    let _ = writeln!(
        out,
        "Keep the existing file path for compatibility, but title the artifact `# Session relay`."
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "Current session snapshot:");
    let _ = writeln!(out, "- Workspace: {}", app.workspace.display());
    let _ = writeln!(out, "- Mode: {}", app.mode.label());
    let _ = writeln!(out, "- Model: {}", app.model_display_label());
    if let Some(focus) = focus {
        let _ = writeln!(out, "- Requested relay focus: {focus}");
    }
    if let Some(quarry) = app.hunt.quarry.as_deref() {
        let _ = writeln!(out, "- Goal objective: {quarry}");
    }
    if let Some(budget) = app.hunt.token_budget {
        let _ = writeln!(out, "- Goal token budget: {budget}");
    }
    if let Ok(todos) = app.todos.try_lock() {
        let snapshot = todos.snapshot();
        if !snapshot.items.is_empty() {
            let _ = writeln!(
                out,
                "\nWork checklist (primary progress surface, {}% complete):",
                snapshot.completion_pct
            );
            for item in snapshot.items {
                let _ = writeln!(
                    out,
                    "- #{} [{}] {}",
                    item.id,
                    item.status.as_str(),
                    item.content
                );
            }
        }
    } else {
        let _ = writeln!(
            out,
            "\nWork checklist: unavailable because the checklist is busy."
        );
    }

    if let Ok(plan) = app.plan_state.try_lock() {
        let snapshot = plan.snapshot();
        if !snapshot.is_empty() {
            let _ = writeln!(out, "\nOptional strategy metadata from update_plan:");
            write_plan_field(&mut out, "Title", snapshot.title.as_deref());
            write_plan_field(&mut out, "Objective", snapshot.objective.as_deref());
            write_plan_field(&mut out, "Context", snapshot.context_summary.as_deref());
            write_plan_field(&mut out, "Explanation", snapshot.explanation.as_deref());
            write_plan_list(&mut out, "Source", &snapshot.sources_used);
            write_plan_list(&mut out, "Critical file", &snapshot.critical_files);
            write_plan_list(&mut out, "Constraint", &snapshot.constraints);
            write_plan_field(
                &mut out,
                "Recommended approach",
                snapshot.recommended_approach.as_deref(),
            );
            write_plan_field(
                &mut out,
                "Verification plan",
                snapshot.verification_plan.as_deref(),
            );
            write_plan_field(
                &mut out,
                "Risks and unknowns",
                snapshot.risks_and_unknowns.as_deref(),
            );
            write_plan_field(
                &mut out,
                "Handoff packet",
                snapshot.handoff_packet.as_deref(),
            );
            for item in snapshot.items {
                let _ = writeln!(out, "- [{}] {}", plan_status_label(&item.status), item.step);
            }
        }
    } else {
        let _ = writeln!(
            out,
            "\nStrategy metadata: unavailable because plan state is busy."
        );
    }

    let _ = writeln!(
        out,
        "\nBefore writing, inspect the current transcript context and any live tool evidence you need. Do not invent test results, file changes, blockers, or decisions."
    );
    let _ = writeln!(
        out,
        "\nUse this compact structure:\n\
         # Session relay\n\
         \n\
         ## Goal\n\
         [the user's objective and any explicit constraints]\n\
         \n\
         ## Current work\n\
         [the active Work checklist item, progress, and what is mid-flight]\n\
         \n\
         ## Files and state\n\
         [changed files, important paths, sub-agents/RLM sessions, commands run]\n\
         \n\
         ## Decisions\n\
         [why key choices were made]\n\
         \n\
         ## Verification\n\
         [what passed, what failed, what was not run]\n\
         \n\
         ## Next action\n\
         [one concrete action for the next thread]"
    );
    let _ = writeln!(
        out,
        "\nKeep it under about 900 words unless the session genuinely needs more. After writing, report the path and the single next action."
    );
    out
}

fn write_plan_field(out: &mut String, label: &str, value: Option<&str>) {
    if let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) {
        let _ = writeln!(out, "- {label}: {value}");
    }
}

fn write_plan_list(out: &mut String, label: &str, values: &[String]) {
    for value in values {
        let value = value.trim();
        if !value.is_empty() {
            let _ = writeln!(out, "- {label}: {value}");
        }
    }
}

fn plan_status_label(status: &crate::tools::plan::StepStatus) -> &'static str {
    match status {
        crate::tools::plan::StepStatus::Pending => "pending",
        crate::tools::plan::StepStatus::InProgress => "in_progress",
        crate::tools::plan::StepStatus::Completed => "completed",
    }
}
