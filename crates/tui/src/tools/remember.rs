//! `remember` tool — model-callable bullet-add into the user memory file.
//!
//! Lets the model itself notice a durable preference, convention, or fact
//! worth keeping across sessions and write it to the user's `memory.md`.
//! The tool is auto-approved and side-effecting only on the user-owned
//! memory file (`~/.deepseek/memory.md` by default), so it doesn't get
//! gated behind the same approval flow as shell or arbitrary file writes.
//!
//! Only registered when `[memory] enabled = true` (or
//! `DEEPSEEK_MEMORY=on`). When disabled, the tool isn't surfaced to the
//! model at all, so prompts that mention `remember` simply fall through.

use async_trait::async_trait;
use serde_json::{Value, json};

use super::spec::{
    ApprovalRequirement, ToolCapability, ToolContext, ToolError, ToolResult, ToolSpec, required_str,
};

/// Tool that appends one bullet to the user memory file.
pub struct RememberTool;

#[async_trait]
impl ToolSpec for RememberTool {
    fn name(&self) -> &'static str {
        "remember"
    }

    fn description(&self) -> &'static str {
        "Append a durable note, active task, or file change log entry to the user memory file \
         so it surfaces in future sessions. Keep notes terse (one sentence). Category must be \
         one of 'preference', 'task', or 'file_change'."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "note": {
                    "type": "string",
                    "description": "The single-sentence durable note, task description, or file change log entry."
                },
                "category": {
                    "type": "string",
                    "enum": ["preference", "task", "file_change"],
                    "description": "The category/section of memory to update. Defaults to 'preference'."
                },
                "scope": {
                    "type": "string",
                    "enum": ["user", "project"],
                    "description": "Whether to remember this in the global user memory ('user') or the project-specific workspace memory ('project'). Defaults to 'user'."
                },
                "file_path": {
                    "type": "string",
                    "description": "The relative or absolute file path. Required when category is 'file_change'."
                },
                "lines": {
                    "type": "string",
                    "description": "The specific line range modified (e.g., '12-34'). Optional; used only for 'file_change'."
                }
            },
            "required": ["note"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WritesFiles]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let note = required_str(&input, "note")?;
        let category = input
            .get("category")
            .and_then(Value::as_str)
            .unwrap_or("preference");
        let scope = input.get("scope").and_then(Value::as_str).unwrap_or("user");
        let file_path = input.get("file_path").and_then(Value::as_str);
        let lines = input.get("lines").and_then(Value::as_str);

        let path = if scope == "project" {
            context.project_memory_path.as_ref().ok_or_else(|| {
                ToolError::execution_failed("project memory is disabled or unavailable")
            })?
        } else {
            context.memory_path.as_ref().ok_or_else(|| {
                ToolError::execution_failed(
                    "user memory is disabled — set `[memory] enabled = true` in config.toml or \
                     `DEEPSEEK_MEMORY=on` in the environment to enable",
                )
            })?
        };

        match category {
            "preference" => {
                crate::memory::append_preference(path, note).map_err(|err| {
                    ToolError::execution_failed(format!("failed to append preference: {err}"))
                })?;
            }
            "task" => {
                crate::memory::append_task(path, note).map_err(|err| {
                    ToolError::execution_failed(format!("failed to append task: {err}"))
                })?;
            }
            "file_change" => {
                let file = file_path.ok_or_else(|| {
                    ToolError::invalid_input("file_path is required when category is 'file_change'")
                })?;
                crate::memory::append_file_change(path, file, lines, note).map_err(|err| {
                    ToolError::execution_failed(format!("failed to append file change: {err}"))
                })?;
            }
            _ => {
                return Err(ToolError::invalid_input(format!(
                    "unknown category '{}'",
                    category
                )));
            }
        }

        Ok(ToolResult::success(format!(
            "remembered in category '{category}' (scope: {scope}): {}",
            note.trim_start_matches('#').trim()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn ctx_with_memory(path: PathBuf) -> ToolContext {
        let mut ctx = ToolContext::new(path.parent().unwrap_or_else(|| std::path::Path::new(".")));
        ctx.memory_path = Some(path);
        ctx
    }

    #[tokio::test]
    async fn returns_error_when_memory_disabled() {
        let tmp = tempdir().unwrap();
        let mut ctx = ToolContext::new(tmp.path());
        ctx.memory_path = None;

        let tool = RememberTool;
        let err = tool
            .execute(json!({"note": "use 4 spaces for indentation"}), &ctx)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("memory is disabled"), "{err}");
    }

    #[tokio::test]
    async fn appends_preference_to_memory_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("memory.md");
        let ctx = ctx_with_memory(path.clone());

        let tool = RememberTool;
        let result = tool
            .execute(
                json!({"note": "use 4 spaces for indentation", "category": "preference"}),
                &ctx,
            )
            .await
            .expect("ok");
        assert!(result.success);
        assert!(result.content.contains("4 spaces"));

        let body = std::fs::read_to_string(&path).expect("read");
        assert!(body.contains("## Preferences"));
        assert!(body.contains("4 spaces"));
    }

    #[tokio::test]
    async fn appends_task_to_memory_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("memory.md");
        let ctx = ctx_with_memory(path.clone());

        let tool = RememberTool;
        let result = tool
            .execute(json!({"note": "Write tests", "category": "task"}), &ctx)
            .await
            .expect("ok");
        assert!(result.success);

        let body = std::fs::read_to_string(&path).expect("read");
        assert!(body.contains("## Active Tasks"));
        assert!(body.contains("[ ]"));
        assert!(body.contains("Write tests"));
    }

    #[tokio::test]
    async fn appends_file_change_to_memory_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("memory.md");
        let ctx = ctx_with_memory(path.clone());

        let tool = RememberTool;
        let result = tool
            .execute(
                json!({
                    "note": "Refactored parsing logic",
                    "category": "file_change",
                    "file_path": "crates/tui/src/parser.rs",
                    "lines": "50-75"
                }),
                &ctx,
            )
            .await
            .expect("ok");
        assert!(result.success);

        let body = std::fs::read_to_string(&path).expect("read");
        assert!(body.contains("## File Change Log"));
        assert!(body.contains("[crates/tui/src/parser.rs:50-75]"));
        assert!(body.contains("Refactored parsing logic"));
    }

    #[tokio::test]
    async fn appends_preference_to_project_memory_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("project-memory.md");
        let mut ctx = ToolContext::new(tmp.path());
        ctx.project_memory_path = Some(path.clone());

        let tool = RememberTool;
        let result = tool
            .execute(
                json!({"note": "use spaces for project", "category": "preference", "scope": "project"}),
                &ctx,
            )
            .await
            .expect("ok");
        assert!(result.success);
        assert!(result.content.contains("use spaces for project"));
        assert!(result.content.contains("scope: project"));

        let body = std::fs::read_to_string(&path).expect("read");
        assert!(body.contains("## Preferences"));
        assert!(body.contains("use spaces for project"));
    }
}
