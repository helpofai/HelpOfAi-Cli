//! Headless web inspection and browser console error listener tool.
//!
//! Enables the AI agent to inspect local website instances (`localhost:3000`, `127.0.0.1:8080`,
//! and local LAN IPs like `192.168.x.x`) or remote websites using a headless browser.
//! Captures rendered DOM, console logs, JavaScript runtime exceptions, and failed network
//! requests, and correlates errors with local workspace source files for automated fixing.

use anyhow::Result;
use async_trait::async_trait;
use helpofai_aios::web_inspector::{WebInspectEngine, WebInspectOptions, inspect_web_page};
use serde_json::{Value, json};

use super::spec::{
    ApprovalRequirement, ToolCapability, ToolContext, ToolError, ToolResult, ToolSpec,
    optional_bool, optional_str, optional_u64, required_str,
};

pub struct WebInspectTool;

#[async_trait]
impl ToolSpec for WebInspectTool {
    fn name(&self) -> &'static str {
        "web_inspect"
    }

    fn description(&self) -> &'static str {
        "Inspect a local or remote web application (e.g. http://localhost:3000, http://127.0.0.1:8080, or local LAN IP http://192.168.x.x:port) using a headless browser. Captures rendered page title/text, browser console errors (console.error, console.warn), JavaScript runtime exceptions, and failed network requests (HTTP 4xx/5xx, CORS). Automatically correlates errors with workspace source files (.js, .jsx, .ts, .tsx, .vue, .svelte, .html) and extracts code snippets for fast debugging and auto-fixing."
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
                    "description": "Milliseconds to wait after page load for client-side JavaScript hydration and async requests (default: 1500)."
                },
                "engine": {
                    "type": "string",
                    "enum": ["auto", "cdp", "playwright", "puppeteer", "http"],
                    "description": "Browser inspection engine: 'auto' (recommended), 'cdp' (native Chrome/Edge DevTools), 'playwright', 'puppeteer', or 'http' (lightweight probe)."
                },
                "correlate_workspace": {
                    "type": "boolean",
                    "description": "Whether to search the workspace to correlate browser errors with source code files (default: true)."
                }
            },
            "required": ["url"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::Network, ToolCapability::ReadOnly]
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto
    }

    async fn execute(&self, input: Value, context: &ToolContext) -> Result<ToolResult, ToolError> {
        let url = required_str(&input, "url")?;
        let wait_ms = optional_u64(&input, "wait_ms", 1500);
        let engine_str = optional_str(&input, "engine").unwrap_or("auto");
        let correlate_workspace = optional_bool(&input, "correlate_workspace", true);

        let options = WebInspectOptions {
            url: url.to_string(),
            wait_ms,
            engine: WebInspectEngine::from(engine_str),
            correlate_workspace,
        };

        let workspace = context.workspace.clone();

        // Run inspection off the async thread to allow synchronous process execution
        let report =
            tokio::task::spawn_blocking(move || inspect_web_page(&options, Some(&workspace)))
                .await
                .map_err(|e| ToolError::execution_failed(format!("Task spawn failed: {e}")))?
                .map_err(|e| ToolError::execution_failed(format!("Web inspection failed: {e}")))?;

        ToolResult::json(&report).map_err(|e| ToolError::execution_failed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_inspect_spec() {
        let tool = WebInspectTool;
        assert_eq!(tool.name(), "web_inspect");
        assert_eq!(tool.approval_requirement(), ApprovalRequirement::Auto);
        assert!(tool.capabilities().contains(&ToolCapability::Network));
        assert!(tool.capabilities().contains(&ToolCapability::ReadOnly));
    }
}
