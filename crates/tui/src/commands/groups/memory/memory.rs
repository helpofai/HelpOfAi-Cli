//! `/memory` slash command — inspect and edit the user memory file.
//!
//! When the user-memory feature is opted-in (`[memory] enabled = true` in
//! config or `DEEPSEEK_MEMORY=on` in the environment), `/memory` shows
//! the current memory file path and contents inline. Subcommands let the
//! user clear, open, search, or track tasks in the file:
//!
//! - `/memory` — show path + content
//! - `/memory show` — alias for the no-arg form
//! - `/memory clear` — replace the file contents with an empty marker
//! - `/memory path` — show only the resolved path
//! - `/memory search <query>` — search bullets in memory file
//! - `/memory task [list|add|complete]` — list, add, or complete tasks
//! - `/memory help` — show command-specific help and the resolved path

use std::fs;
use std::path::Path;

use super::CommandResult;
use crate::tui::app::App;

const MEMORY_USAGE: &str = "/memory [show|path|clear|edit|search|task|help]";

fn memory_help(path: &Path) -> String {
    format!(
        "Inspect or manage your persistent user-memory file.\n\n\
         Usage: {MEMORY_USAGE}\n\n\
         Current path: {}\n\n\
         Subcommands:\n\
           /memory                  Show the resolved path and current contents\n\
           /memory show             Alias for the no-arg form\n\
           /memory path             Print just the resolved path\n\
           /memory clear            Replace the file contents with an empty marker\n\
           /memory edit             Print the editor command for this file\n\
           /memory search <query>   Search bullets in your memory file\n\
           /memory task             List active memory tasks\n\
           /memory task add <desc>  Add a new memory task\n\
           /memory task complete <n> Mark task #n as completed\n\
           /memory help             Show this help\n\n\
         Quick capture: type `# foo` in the composer to append a timestamped\n\
         bullet without firing a turn.",
        path.display()
    )
}

pub fn memory(app: &mut App, arg: Option<&str>) -> CommandResult {
    if !app.use_memory {
        return CommandResult::error(
            "user memory is disabled. Enable with `[memory] enabled = true` in `~/.helpofai/config.toml` or `DEEPSEEK_MEMORY=on` in your environment, then restart the TUI.",
        );
    }

    let path = app.memory_path.clone();
    let trimmed_arg = arg.unwrap_or("show").trim();
    let parts: Vec<&str> = trimmed_arg.split_whitespace().collect();
    let sub = parts.first().copied().unwrap_or("show");

    match sub {
        "" | "show" => {
            let body = match fs::read_to_string(&path) {
                Ok(text) if text.trim().is_empty() => format!(
                    "{}\n(empty — add via `# foo` from the composer or have the model use the `remember` tool)",
                    path.display()
                ),
                Ok(text) => format!("{}\n\n{}", path.display(), text.trim_end()),
                Err(_) => format!(
                    "{}\n(file does not exist yet — add via `# foo` from the composer to create it)",
                    path.display()
                ),
            };
            CommandResult::message(body)
        }
        "path" => CommandResult::message(path.display().to_string()),
        "clear" => match fs::write(&path, "") {
            Ok(()) => CommandResult::message(format!("memory cleared: {}", path.display())),
            Err(err) => CommandResult::error(format!("failed to clear {}: {err}", path.display())),
        },
        "edit" => CommandResult::message(format!(
            "to edit your memory file, run:\n\n  ${{VISUAL:-${{EDITOR:-vi}}}} {}",
            path.display()
        )),
        "help" => CommandResult::message(memory_help(&path)),
        "search" => {
            let query = parts[1..].join(" ");
            if query.is_empty() {
                return CommandResult::error(
                    "please specify a search term: `/memory search <query>`",
                );
            }
            let text = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(_) => return CommandResult::error("memory file does not exist yet"),
            };
            let mut matches = Vec::new();
            for (idx, line) in text.lines().enumerate() {
                let trimmed = line.trim();
                if !trimmed.is_empty()
                    && !trimmed.starts_with("#")
                    && trimmed.to_lowercase().contains(&query.to_lowercase())
                {
                    matches.push(format!("  Line {}: {}", idx + 1, trimmed));
                }
            }
            if matches.is_empty() {
                CommandResult::message(format!("No matches found for query: '{query}'"))
            } else {
                CommandResult::message(format!(
                    "Found {} matches in memory file:\n\n{}",
                    matches.len(),
                    matches.join("\n")
                ))
            }
        }
        "task" => {
            let task_sub = parts.get(1).copied().unwrap_or("list");
            match task_sub {
                "list" | "" => {
                    let tasks = crate::memory::list_tasks(&path);
                    match tasks {
                        Ok(t) => {
                            if t.is_empty() {
                                CommandResult::message("No active tasks found in memory.")
                            } else {
                                let mut list = Vec::new();
                                for (i, (completed, desc)) in t.iter().enumerate() {
                                    let marker = if *completed { "[x]" } else { "[ ]" };
                                    list.push(format!("  {} {} {}", i + 1, marker, desc));
                                }
                                CommandResult::message(format!(
                                    "Active Tasks:\n\n{}",
                                    list.join("\n")
                                ))
                            }
                        }
                        Err(err) => CommandResult::error(format!("failed to read tasks: {err}")),
                    }
                }
                "add" => {
                    let desc = parts[2..].join(" ");
                    if desc.is_empty() {
                        return CommandResult::error(
                            "please specify a task description: `/memory task add <description>`",
                        );
                    }
                    match crate::memory::append_task(&path, &desc) {
                        Ok(()) => CommandResult::message(format!("Added task: {desc}")),
                        Err(err) => CommandResult::error(format!("failed to add task: {err}")),
                    }
                }
                "complete" => {
                    let index_str = parts.get(2).copied().unwrap_or("");
                    let index = index_str.parse::<usize>().ok();
                    let Some(idx) = index else {
                        return CommandResult::error(
                            "please specify a task index: `/memory task complete <index>`",
                        );
                    };
                    match crate::memory::complete_task(&path, idx) {
                        Ok(Some(line)) => {
                            CommandResult::message(format!("Completed task: {}", line.trim()))
                        }
                        Ok(None) => {
                            CommandResult::error(format!("task index {idx} is out of bounds"))
                        }
                        Err(err) => CommandResult::error(format!("failed to complete task: {err}")),
                    }
                }
                other => CommandResult::error(format!(
                    "unknown task subcommand `{other}`. Try `/memory task [list|add|complete]`"
                )),
            }
        }
        _ => CommandResult::error(format!(
            "unknown subcommand `{sub}`. Try `/memory help`.\n\n{}",
            memory_help(&path)
        )),
    }
}

const PROJECT_MEMORY_USAGE: &str = "/projectmemory [show|path|clear|edit|search|task|help]";

fn project_memory_help(path: &Path) -> String {
    format!(
        "Inspect or manage your project-specific memory file.\n\n\
         Usage: {PROJECT_MEMORY_USAGE}\n\n\
         Current path: {}\n\n\
         Subcommands:\n\
           /projectmemory                  Show the resolved path and current contents\n\
           /projectmemory show             Alias for the no-arg form\n\
           /projectmemory path             Print just the resolved path\n\
           /projectmemory clear            Replace the file contents with an empty marker\n\
           /projectmemory edit             Print the editor command for this file\n\
           /projectmemory search <query>   Search bullets in your project memory file\n\
           /projectmemory task             List active project memory tasks\n\
           /projectmemory task add <desc>  Add a new project memory task\n\
           /projectmemory task complete <n> Mark task #n as completed\n\
           /projectmemory help             Show this help\n\n\
         Quick capture: type `# foo` in the composer to append a timestamped\n\
         bullet to user memory, or use `/projectmemory task add <desc>` or have the model\n\
         use the `remember` tool with scope='project'.",
        path.display()
    )
}

pub fn project_memory(app: &mut App, arg: Option<&str>) -> CommandResult {
    let Some(path) = app.project_memory_path.clone() else {
        return CommandResult::error(
            "project memory is disabled or unavailable. Enable memory with `[memory] enabled = true` in `~/.helpofai/config.toml` or `DEEPSEEK_MEMORY=on` in your environment, then restart the TUI.",
        );
    };

    let trimmed_arg = arg.unwrap_or("show").trim();
    let parts: Vec<&str> = trimmed_arg.split_whitespace().collect();
    let sub = parts.first().copied().unwrap_or("show");

    match sub {
        "" | "show" => {
            let body = match fs::read_to_string(&path) {
                Ok(text) if text.trim().is_empty() => format!(
                    "{}\n(empty — use `/projectmemory task add <desc>` or have the model use the `remember` tool with scope='project')",
                    path.display()
                ),
                Ok(text) => format!("{}\n\n{}", path.display(), text.trim_end()),
                Err(_) => format!(
                    "{}\n(file does not exist yet — use `/projectmemory task add <desc>` to create it or have the model use the `remember` tool with scope='project')",
                    path.display()
                ),
            };
            CommandResult::message(body)
        }
        "path" => CommandResult::message(path.display().to_string()),
        "clear" => match fs::write(&path, "") {
            Ok(()) => CommandResult::message(format!("project memory cleared: {}", path.display())),
            Err(err) => CommandResult::error(format!("failed to clear {}: {err}", path.display())),
        },
        "edit" => CommandResult::message(format!(
            "to edit your project memory file, run:\n\n  ${{VISUAL:-${{EDITOR:-vi}}}} {}",
            path.display()
        )),
        "help" => CommandResult::message(project_memory_help(&path)),
        "search" => {
            let query = parts[1..].join(" ");
            if query.is_empty() {
                return CommandResult::error(
                    "please specify a search term: `/projectmemory search <query>`",
                );
            }
            let text = match fs::read_to_string(&path) {
                Ok(t) => t,
                Err(_) => return CommandResult::error("project memory file does not exist yet"),
            };
            let mut matches = Vec::new();
            for (idx, line) in text.lines().enumerate() {
                let trimmed = line.trim();
                if !trimmed.is_empty()
                    && !trimmed.starts_with("#")
                    && trimmed.to_lowercase().contains(&query.to_lowercase())
                {
                    matches.push(format!("  Line {}: {}", idx + 1, trimmed));
                }
            }
            if matches.is_empty() {
                CommandResult::message(format!("No matches found for query: '{query}'"))
            } else {
                CommandResult::message(format!(
                    "Found {} matches in project memory file:\n\n{}",
                    matches.len(),
                    matches.join("\n")
                ))
            }
        }
        "task" => {
            let task_sub = parts.get(1).copied().unwrap_or("list");
            match task_sub {
                "list" | "" => {
                    let tasks = crate::memory::list_tasks(&path);
                    match tasks {
                        Ok(t) => {
                            if t.is_empty() {
                                CommandResult::message("No active tasks found in project memory.")
                            } else {
                                let mut list = Vec::new();
                                for (i, (completed, desc)) in t.iter().enumerate() {
                                    let marker = if *completed { "[x]" } else { "[ ]" };
                                    list.push(format!("  {} {} {}", i + 1, marker, desc));
                                }
                                CommandResult::message(format!(
                                    "Active Tasks:\n\n{}",
                                    list.join("\n")
                                ))
                            }
                        }
                        Err(err) => CommandResult::error(format!("failed to read tasks: {err}")),
                    }
                }
                "add" => {
                    let desc = parts[2..].join(" ");
                    if desc.is_empty() {
                        return CommandResult::error(
                            "please specify a task description: `/projectmemory task add <description>`",
                        );
                    }
                    match crate::memory::append_task(&path, &desc) {
                        Ok(()) => CommandResult::message(format!("Added task: {desc}")),
                        Err(err) => CommandResult::error(format!("failed to add task: {err}")),
                    }
                }
                "complete" => {
                    let index_str = parts.get(2).copied().unwrap_or("");
                    let index = index_str.parse::<usize>().ok();
                    let Some(idx) = index else {
                        return CommandResult::error(
                            "please specify a task index: `/projectmemory task complete <index>`",
                        );
                    };
                    match crate::memory::complete_task(&path, idx) {
                        Ok(Some(line)) => {
                            CommandResult::message(format!("Completed task: {}", line.trim()))
                        }
                        Ok(None) => {
                            CommandResult::error(format!("task index {idx} is out of bounds"))
                        }
                        Err(err) => CommandResult::error(format!("failed to complete task: {err}")),
                    }
                }
                other => CommandResult::error(format!(
                    "unknown task subcommand `{other}`. Try `/projectmemory task [list|add|complete]`"
                )),
            }
        }
        _ => CommandResult::error(format!(
            "unknown subcommand `{sub}`. Try `/projectmemory help`.\n\n{}",
            project_memory_help(&path)
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::tui::app::{App, TuiOptions};
    use tempfile::TempDir;

    fn create_test_app_with_memory(tmpdir: &TempDir, use_memory: bool) -> App {
        let options = TuiOptions {
            model: "deepseek-v4-pro".to_string(),
            workspace: tmpdir.path().to_path_buf(),
            config_path: None,
            config_profile: None,
            allow_shell: false,
            use_alt_screen: true,
            use_mouse_capture: false,
            use_bracketed_paste: true,
            max_subagents: 1,
            skills_dir: tmpdir.path().join("skills"),
            memory_path: tmpdir.path().join("memory.md"),
            notes_path: tmpdir.path().join("notes.txt"),
            mcp_config_path: tmpdir.path().join("mcp.json"),
            use_memory,
            start_in_agent_mode: false,
            skip_onboarding: true,
            yolo: false,
            resume_session_id: None,
            initial_input: None,
        };
        App::new(options, &Config::default())
    }

    #[test]
    fn memory_help_lists_subcommands_and_resolved_path() {
        let tmpdir = TempDir::new().expect("tempdir");
        let mut app = create_test_app_with_memory(&tmpdir, true);
        let result = memory(&mut app, Some("help"));
        let msg = result.message.expect("help should return text");
        assert!(msg.contains("Usage: /memory [show|path|clear|edit|search|task|help]"));
        assert!(msg.contains("/memory edit"));
        assert!(msg.contains(app.memory_path.to_string_lossy().as_ref()));
    }

    #[test]
    fn memory_search_and_task_work() {
        let tmpdir = TempDir::new().expect("tempdir");
        let mut app = create_test_app_with_memory(&tmpdir, true);

        // Add preference
        crate::memory::append_preference(&app.memory_path, "always use spaces").unwrap();
        // Add task
        memory(&mut app, Some("task add Implement memory search"))
            .message
            .unwrap();

        // Search
        let search_result = memory(&mut app, Some("search spaces")).message.unwrap();
        assert!(search_result.contains("always use spaces"));

        // List tasks
        let task_list = memory(&mut app, Some("task list")).message.unwrap();
        assert!(task_list.contains("[ ]"));
        assert!(task_list.contains("Implement memory search"));

        // Complete task
        memory(&mut app, Some("task complete 1")).message.unwrap();
        let task_list_completed = memory(&mut app, Some("task list")).message.unwrap();
        assert!(task_list_completed.contains("[x]"));
    }

    #[test]
    fn project_memory_help_lists_subcommands_and_resolved_path() {
        let tmpdir = TempDir::new().expect("tempdir");
        let mut app = create_test_app_with_memory(&tmpdir, true);
        let result = project_memory(&mut app, Some("help"));
        let msg = result.message.expect("help should return text");
        assert!(msg.contains("Usage: /projectmemory [show|path|clear|edit|search|task|help]"));
        assert!(msg.contains("/projectmemory edit"));
        assert!(
            msg.contains(
                app.project_memory_path
                    .as_ref()
                    .unwrap()
                    .to_string_lossy()
                    .as_ref()
            )
        );
    }

    #[test]
    fn project_memory_search_and_task_work() {
        let tmpdir = TempDir::new().expect("tempdir");
        let mut app = create_test_app_with_memory(&tmpdir, true);
        let p_path = app.project_memory_path.clone().unwrap();

        // Add preference
        crate::memory::append_preference(&p_path, "always use tabs for project").unwrap();
        // Add task
        project_memory(&mut app, Some("task add Implement project search"))
            .message
            .unwrap();

        // Search
        let search_result = project_memory(&mut app, Some("search tabs"))
            .message
            .unwrap();
        assert!(search_result.contains("always use tabs for project"));

        // List tasks
        let task_list = project_memory(&mut app, Some("task list")).message.unwrap();
        assert!(task_list.contains("[ ]"));
        assert!(task_list.contains("Implement project search"));

        // Complete task
        project_memory(&mut app, Some("task complete 1"))
            .message
            .unwrap();
        let task_list_completed = project_memory(&mut app, Some("task list")).message.unwrap();
        assert!(task_list_completed.contains("[x]"));
    }
}
