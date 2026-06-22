//! Core command area: model/provider selection, help, navigation, and the
//! persistent RLM / sub-agent entry points.

mod anchor;
// This group dir intentionally has a `core.rs` child module with the same
// name. The module_inception allow is a permanent structure rationale, not
// migration scaffolding; see docs/architecture/command-dispatch.md.
#[allow(clippy::module_inception)]
mod core;
mod feedback;
mod hf;
mod hooks;
mod provider;
mod queue;
mod stash;
pub mod voice;

pub(in crate::commands) use self::core::reset_conversation_state;

use crate::commands::CommandResult;
use crate::commands::traits::{Command, CommandGroup, CommandInfo, FunctionCommand};
use crate::localization::MessageId;
use crate::tui::app::{App, AppAction};

pub struct CoreCommands;

impl CommandGroup for CoreCommands {
    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![
            Box::new(FunctionCommand::new(&ANCHOR_INFO, run_anchor)),
            Box::new(FunctionCommand::new(&HELP_INFO, run_help)),
            Box::new(FunctionCommand::new(&CLEAR_INFO, run_clear)),
            Box::new(FunctionCommand::new(&EXIT_INFO, run_exit)),
            Box::new(FunctionCommand::new(&MODEL_INFO, run_model)),
            Box::new(FunctionCommand::new(&MODELS_INFO, run_models)),
            Box::new(FunctionCommand::new(&PROVIDER_INFO, run_provider)),
            Box::new(FunctionCommand::new(&QUEUE_INFO, run_queue)),
            Box::new(FunctionCommand::new(&STASH_INFO, run_stash)),
            Box::new(FunctionCommand::new(&HOOKS_INFO, run_hooks)),
            Box::new(FunctionCommand::new(&SUBAGENTS_INFO, run_subagents)),
            Box::new(FunctionCommand::new(&AGENT_INFO, run_agent)),
            Box::new(FunctionCommand::new(&SWARM_INFO, run_swarm)),
            Box::new(FunctionCommand::new(&LINKS_INFO, run_links)),
            Box::new(FunctionCommand::new(&FEEDBACK_INFO, run_feedback)),
            Box::new(FunctionCommand::new(&HF_INFO, run_hf)),
            Box::new(FunctionCommand::new(&HOME_INFO, run_home)),
            Box::new(FunctionCommand::new(&WORKSPACE_INFO, run_workspace)),
            Box::new(FunctionCommand::new(&PROFILE_INFO, run_profile)),
            Box::new(FunctionCommand::new(&RLM_INFO, run_rlm)),
            Box::new(FunctionCommand::new(&TRANSLATE_INFO, run_translate)),
            Box::new(FunctionCommand::new(&VOICE_INFO, run_voice)),
            Box::new(FunctionCommand::new(&VOICE_SEND_INFO, run_voice_send)),
            Box::new(FunctionCommand::new(&VOICE_CONTROL_INFO, run_voice_control)),
        ]
    }
}

static ANCHOR_INFO: CommandInfo = CommandInfo {
    name: "anchor",
    aliases: &["maodian"],
    usage: "/anchor <text> | /anchor list | /anchor remove <n>",
    description_id: MessageId::CmdAnchorDescription,
};
static HELP_INFO: CommandInfo = CommandInfo {
    name: "help",
    aliases: &["?", "bangzhu", "帮助"],
    usage: "/help [command]",
    description_id: MessageId::CmdHelpDescription,
};
static CLEAR_INFO: CommandInfo = CommandInfo {
    name: "clear",
    aliases: &["qingping"],
    usage: "/clear",
    description_id: MessageId::CmdClearDescription,
};
static EXIT_INFO: CommandInfo = CommandInfo {
    name: "exit",
    aliases: &["quit", "q", "tuichu"],
    usage: "/exit",
    description_id: MessageId::CmdExitDescription,
};
static MODEL_INFO: CommandInfo = CommandInfo {
    name: "model",
    aliases: &["moxing"],
    usage: "/model [name]",
    description_id: MessageId::CmdModelDescription,
};
static MODELS_INFO: CommandInfo = CommandInfo {
    name: "models",
    aliases: &["moxingliebiao"],
    usage: "/models",
    description_id: MessageId::CmdModelsDescription,
};
static PROVIDER_INFO: CommandInfo = CommandInfo {
    name: "provider",
    aliases: &[],
    usage: "/provider [name] [model]",
    description_id: MessageId::CmdProviderDescription,
};
static QUEUE_INFO: CommandInfo = CommandInfo {
    name: "queue",
    aliases: &["queued"],
    usage: "/queue [list|send <n>|edit <n>|drop <n>|clear]",
    description_id: MessageId::CmdQueueDescription,
};
static STASH_INFO: CommandInfo = CommandInfo {
    name: "stash",
    aliases: &["park"],
    usage: "/stash [list|pop|clear]",
    description_id: MessageId::CmdStashDescription,
};
static HOOKS_INFO: CommandInfo = CommandInfo {
    name: "hooks",
    aliases: &["hook", "gouzi"],
    usage: "/hooks [list|events]",
    description_id: MessageId::CmdHooksDescription,
};
static SUBAGENTS_INFO: CommandInfo = CommandInfo {
    name: "subagents",
    aliases: &["agents", "zhinengti"],
    usage: "/subagents",
    description_id: MessageId::CmdSubagentsDescription,
};
static AGENT_INFO: CommandInfo = CommandInfo {
    name: "agent",
    aliases: &["daili"],
    usage: "/agent [N] <task>",
    description_id: MessageId::CmdAgentDescription,
};
static SWARM_INFO: CommandInfo = CommandInfo {
    name: "swarm",
    aliases: &["fanout", "qun"],
    usage: "/swarm [N] <task>",
    description_id: MessageId::CmdSwarmDescription,
};
static LINKS_INFO: CommandInfo = CommandInfo {
    name: "links",
    aliases: &["dashboard", "api", "lianjie"],
    usage: "/links",
    description_id: MessageId::CmdLinksDescription,
};
static FEEDBACK_INFO: CommandInfo = CommandInfo {
    name: "feedback",
    aliases: &[],
    usage: "/feedback [bug|feature|security]",
    description_id: MessageId::CmdFeedbackDescription,
};
static HF_INFO: CommandInfo = CommandInfo {
    name: "hf",
    aliases: &["huggingface"],
    usage: "/hf [mcp <status|setup>|concepts]",
    description_id: MessageId::CmdHfDescription,
};
static HOME_INFO: CommandInfo = CommandInfo {
    name: "home",
    aliases: &["stats", "overview", "zhuye", "shouye"],
    usage: "/home",
    description_id: MessageId::CmdHomeDescription,
};
static WORKSPACE_INFO: CommandInfo = CommandInfo {
    name: "workspace",
    aliases: &["cwd"],
    usage: "/workspace [path]",
    description_id: MessageId::CmdWorkspaceDescription,
};
static PROFILE_INFO: CommandInfo = CommandInfo {
    name: "profile",
    aliases: &["dangan"],
    usage: "/profile <name>",
    description_id: MessageId::CmdHelpDescription,
};
static RLM_INFO: CommandInfo = CommandInfo {
    name: "rlm",
    aliases: &["recursive", "digui"],
    usage: "/rlm [N] <file_or_text>",
    description_id: MessageId::CmdRlmDescription,
};
static TRANSLATE_INFO: CommandInfo = CommandInfo {
    name: "translate",
    aliases: &["translation", "transale"],
    usage: "/translate",
    description_id: MessageId::CmdTranslateDescription,
};
static VOICE_INFO: CommandInfo = CommandInfo {
    name: "voice",
    aliases: &["yuyin", "语音"],
    usage: "/voice",
    description_id: MessageId::CmdVoiceDescription,
};
static VOICE_SEND_INFO: CommandInfo = CommandInfo {
    name: "voicesend",
    aliases: &["voice-send", "yuyinsend", "语音发送"],
    usage: "/voicesend",
    description_id: MessageId::CmdVoiceSendDescription,
};
static VOICE_CONTROL_INFO: CommandInfo = CommandInfo {
    name: "voicecontrol",
    aliases: &["voice-control", "yuyincontrol", "语音控制"],
    usage: "/voicecontrol",
    description_id: MessageId::CmdVoiceControlDescription,
};

fn run_registered(app: &mut App, name: &str, arg: Option<&str>) -> CommandResult {
    dispatch(app, name, arg).expect("registered core command should dispatch")
}

fn run_anchor(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "anchor", arg)
}
fn run_help(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "help", arg)
}
fn run_clear(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "clear", arg)
}
fn run_exit(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "exit", arg)
}
fn run_model(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "model", arg)
}
fn run_models(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "models", arg)
}
fn run_provider(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "provider", arg)
}
fn run_queue(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "queue", arg)
}
fn run_stash(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "stash", arg)
}
fn run_hooks(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "hooks", arg)
}
fn run_subagents(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "subagents", arg)
}
fn run_agent(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "agent", arg)
}
fn run_swarm(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "swarm", arg)
}
fn run_links(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "links", arg)
}
fn run_feedback(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "feedback", arg)
}
fn run_hf(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "hf", arg)
}
fn run_home(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "home", arg)
}
fn run_workspace(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "workspace", arg)
}
fn run_profile(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "profile", arg)
}
fn run_rlm(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "rlm", arg)
}
fn run_translate(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "translate", arg)
}
fn run_voice(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "voice", arg)
}
fn run_voice_send(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "voicesend", arg)
}
fn run_voice_control(app: &mut App, arg: Option<&str>) -> CommandResult {
    run_registered(app, "voicecontrol", arg)
}

pub(in crate::commands) fn dispatch(
    app: &mut App,
    command: &str,
    arg: Option<&str>,
) -> Option<CommandResult> {
    let result = match command {
        "anchor" | "maodian" => anchor::anchor(app, arg),
        "help" | "?" | "bangzhu" | "帮助" => core::help(app, arg),
        "clear" | "qingping" => core::clear(app),
        "exit" | "quit" | "q" | "tuichu" => core::exit(),
        "model" | "moxing" => core::model(app, arg),
        "models" | "moxingliebiao" => core::models(app),
        "provider" => provider::provider(app, arg),
        "queue" | "queued" => queue::queue(app, arg),
        "stash" | "park" => stash::stash(app, arg),
        "hooks" | "hook" | "gouzi" => hooks::hooks(app, arg),
        "subagents" | "agents" | "zhinengti" => core::subagents(app),
        "agent" | "daili" => agent(app, arg),
        "swarm" | "fanout" | "qun" => swarm(app, arg),
        "links" | "dashboard" | "api" | "lianjie" => core::deepseek_links(app),
        "feedback" => feedback::feedback(app, arg),
        "hf" | "huggingface" => hf::hf(app, arg),
        "home" | "stats" | "overview" | "zhuye" | "shouye" => core::home_dashboard(app),
        "workspace" | "cwd" => core::workspace_switch(app, arg),
        "profile" | "dangan" => core::profile_switch(app, arg),
        "rlm" | "recursive" | "digui" => rlm(app, arg),
        "translate" | "translation" | "transale" => core::translate(app),
        "voice" | "yuyin" | "语音" => voice::voice(app),
        "voicesend" | "voice-send" | "yuyinsend" | "语音发送" => voice::voice_send(app),
        "voicecontrol" | "voice-control" | "yuyincontrol" | "语音控制" => {
            voice::voice_control(app)
        }
        _ => return None,
    };
    Some(result)
}

/// Execute a Recursive Language Model (RLM) turn — Algorithm 1 from
/// Zhang et al. (arXiv:2512.24601).
///
/// The user's prompt text is passed as the argument. It will be stored
/// in the REPL as the `PROMPT` variable. The root LLM will only see
/// metadata about the REPL state, never the prompt text directly.
pub fn rlm(app: &mut App, arg: Option<&str>) -> CommandResult {
    let (max_depth, target) = match parse_depth_prefixed_arg(arg, 1) {
        Ok(parsed) => parsed,
        Err(message) => return CommandResult::error(message),
    };
    let target = match target {
        Some(p) if !p.trim().is_empty() => p.trim().to_string(),
        _ => {
            return CommandResult::error(
                "Usage: /rlm [N] <file_or_text>\n\n\
                 Opens a persistent RLM context with sub_rlm depth N (0-3, default 1)."
                    .to_string(),
            );
        }
    };

    let source_arg = if resolves_to_existing_file(app, &target) {
        format!(r#"file_path: "{target}""#)
    } else {
        format!("content: {target:?}")
    };
    let message = format!(
        "Open and use a persistent RLM session for this request. Call `rlm_open` with name `slash_rlm` and {source_arg}. Then call `rlm_configure` with `sub_rlm_max_depth: {max_depth}`. Use `rlm_eval` to inspect the context through `peek`, `search`, and `chunk`, and call `finalize(...)` from the REPL when ready. If a `var_handle` is returned, use `handle_read` for bounded slices or projections before answering."
    );

    CommandResult::with_message_and_action(
        format!("Opening persistent RLM context at depth {max_depth}..."),
        AppAction::SendMessage(message),
    )
}

/// Open a persistent sub-agent session from a slash command.
pub fn agent(_app: &mut App, arg: Option<&str>) -> CommandResult {
    let (max_depth, task) = match parse_depth_prefixed_arg(arg, 1) {
        Ok(parsed) => parsed,
        Err(message) => return CommandResult::error(message),
    };
    let task = match task {
        Some(task) if !task.trim().is_empty() => task.trim().to_string(),
        _ => {
            return CommandResult::error(
                "Usage: /agent [N] <task>\n\n\
                 Opens a persistent sub-agent session with recursive agent depth N (0-3, default 1).",
            );
        }
    };
    let message = format!(
        "Launch one sub-agent for this task by calling `agent` with name `slash_agent`, `prompt: {task:?}`, and `max_depth: {max_depth}`. Use `handle_read` on the returned transcript_handle if you need more detail. Verify any claimed side effects before reporting success."
    );
    CommandResult::with_message_and_action(
        format!("Opening persistent sub-agent at depth {max_depth}..."),
        AppAction::SendMessage(message),
    )
}

/// Gate the old prompt-only swarm fanout until it can route through durable
/// HelpFlow/Fleet workers (#3218).
pub fn swarm(_app: &mut App, arg: Option<&str>) -> CommandResult {
    let (_max_depth, task) = match parse_depth_prefixed_arg(arg, 1) {
        Ok(parsed) => parsed,
        Err(message) => return CommandResult::error(message),
    };
    if !matches!(task.map(str::trim), Some(task) if !task.is_empty()) {
        return CommandResult::error(
            "Usage: /swarm [N] <task>\n\n\
             /swarm is currently gated. Use /goal for a persistent objective \
             or /agent for a single sub-agent while durable Fleet-backed \
             swarm workers are still landing.",
        );
    }
    CommandResult::error(
        "/swarm is gated in v0.8.61: prompt-only agent fanout is disabled until the durable Train-3 worker/goal re-dispatch substrate lands. Use /goal for the persistent objective or /agent [N] <task> for one bounded sub-agent.",
    )
}

fn parse_depth_prefixed_arg(
    arg: Option<&str>,
    default_depth: u32,
) -> Result<(u32, Option<&str>), String> {
    let Some(raw) = arg.map(str::trim).filter(|raw| !raw.is_empty()) else {
        return Ok((default_depth, None));
    };
    let mut parts = raw.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or_default();
    if first.chars().all(|ch| ch.is_ascii_digit()) {
        let depth: u32 = first
            .parse()
            .map_err(|_| "Depth must be an integer from 0 to 3".to_string())?;
        if depth > 3 {
            return Err("Depth must be between 0 and 3".to_string());
        }
        Ok((depth, parts.next().map(str::trim)))
    } else {
        Ok((default_depth, Some(raw)))
    }
}

fn resolves_to_existing_file(app: &App, input: &str) -> bool {
    let path = std::path::Path::new(input);
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        app.workspace.join(path)
    };
    candidate.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_app() -> App {
        let options = crate::tui::app::TuiOptions {
            model: "deepseek-v4-pro".to_string(),
            workspace: std::path::PathBuf::from("/tmp/test-workspace"),
            config_path: None,
            config_profile: None,
            allow_shell: false,
            use_alt_screen: true,
            use_mouse_capture: false,
            use_bracketed_paste: true,
            max_subagents: 1,
            skills_dir: std::path::PathBuf::from("/tmp/test-skills"),
            memory_path: std::path::PathBuf::from("memory.md"),
            notes_path: std::path::PathBuf::from("notes.txt"),
            mcp_config_path: std::path::PathBuf::from("mcp.json"),
            use_memory: false,
            start_in_agent_mode: false,
            skip_onboarding: true,
            initial_input: None,
            resume_session_id: None,
            yolo: false,
        };
        App::new(options, &crate::config::Config::default())
    }

    #[test]
    fn swarm_is_gated_until_durable_worker_substrate_lands() {
        let mut app = create_test_app();
        let result = swarm(&mut app, Some("inspect five files"));

        assert!(result.is_error);
        assert!(result.action.is_none());
        assert!(
            result
                .message
                .as_deref()
                .unwrap_or_default()
                .contains("gated")
        );
        assert!(
            result
                .message
                .as_deref()
                .unwrap_or_default()
                .contains("Train-3")
        );
    }
}
