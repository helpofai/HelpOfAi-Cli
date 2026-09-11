//! # AIOS Web Inspector & Console Error Listener
//!
//! Provides automated web application inspection for local network (`localhost`, `127.0.0.1`,
//! and private LAN IPs like `192.168.x.x`) and remote websites.
//!
//! Features:
//! 1. Headless Browser Page Inspection: Render full DOM, extract title, and visible text.
//! 2. Console & Network Error Listener: Capture `console.error`, `console.warn`, uncaught
//!    JavaScript exceptions, stack traces, and failed network requests (HTTP 4xx/5xx, CORS).
//! 3. Automated Error Diagnosis & Fix Loop: Correlates client-side errors with local workspace
//!    source files (`.js`, `.jsx`, `.ts`, `.tsx`, `.vue`, `.svelte`, `.html`) and extracts
//!    exact line snippets for the AI agent to patch.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Supported browser inspection engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WebInspectEngine {
    #[default]
    Auto,
    Cdp,
    Playwright,
    Puppeteer,
    Http,
}

impl std::str::FromStr for WebInspectEngine {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().to_lowercase().as_str() {
            "cdp" | "chrome" | "edge" => Self::Cdp,
            "playwright" => Self::Playwright,
            "puppeteer" => Self::Puppeteer,
            "http" | "curl" => Self::Http,
            _ => Self::Auto,
        })
    }
}

impl From<&str> for WebInspectEngine {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or_default()
    }
}

impl WebInspectEngine {
    pub fn parse_engine(s: &str) -> Self {
        s.parse().unwrap_or_default()
    }
}

/// Options for web page inspection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebInspectOptions {
    pub url: String,
    #[serde(default = "default_wait_ms")]
    pub wait_ms: u64,
    #[serde(default)]
    pub engine: WebInspectEngine,
    #[serde(default = "default_correlate")]
    pub correlate_workspace: bool,
}

fn default_wait_ms() -> u64 {
    1500
}

fn default_correlate() -> bool {
    true
}

impl Default for WebInspectOptions {
    fn default() -> Self {
        Self {
            url: String::new(),
            wait_ms: default_wait_ms(),
            engine: WebInspectEngine::Auto,
            correlate_workspace: true,
        }
    }
}

/// Captured console error, warning, or uncaught exception.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsoleError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

/// Captured network request failure (4xx, 5xx, CORS, or blocked).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkError {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    pub error_text: String,
}

/// A matched workspace source file associated with a browser error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileDiagnosticMatch {
    pub file_path: String,
    pub line_number: u32,
    pub error_message: String,
    pub snippet: String,
    pub suggested_fix: String,
}

/// The final structured report returned by the Web Inspector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebInspectReport {
    pub url: String,
    pub title: Option<String>,
    pub status: Option<u16>,
    pub engine_used: String,
    pub console_errors: Vec<ConsoleError>,
    pub network_errors: Vec<NetworkError>,
    pub text_snippet: String,
    pub correlated_files: Vec<FileDiagnosticMatch>,
    pub summary: String,
}

/// Locates a supported browser executable on the current system (Edge, Chrome, Brave, Chromium).
pub fn find_system_browser() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let candidates = [
            "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
            "C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
        ];
        for path in &candidates {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let user_edge =
                PathBuf::from(&local_app_data).join("Microsoft\\Edge\\Application\\msedge.exe");
            if user_edge.exists() {
                return Some(user_edge);
            }
            let user_chrome =
                PathBuf::from(&local_app_data).join("Google\\Chrome\\Application\\chrome.exe");
            if user_chrome.exists() {
                return Some(user_chrome);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let candidates = [
            "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
        ];
        for path in &candidates {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let candidates = [
            "google-chrome",
            "google-chrome-stable",
            "chromium",
            "chromium-browser",
            "microsoft-edge",
            "microsoft-edge-stable",
            "brave-browser",
        ];
        for bin in &candidates {
            if let Ok(output) = Command::new("which").arg(bin).output() {
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path_str.is_empty() {
                        return Some(PathBuf::from(path_str));
                    }
                }
            }
        }
    }

    None
}

/// Checks if Node.js is installed and executable.
pub fn has_node() -> bool {
    Command::new("node")
        .arg("-v")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Executes browser inspection and returns a full diagnostic report.
pub fn inspect_web_page(
    options: &WebInspectOptions,
    workspace: Option<&Path>,
) -> anyhow::Result<WebInspectReport> {
    let url = options.url.trim();
    if url.is_empty() {
        anyhow::bail!("Target URL cannot be empty");
    }

    // Auto-prepend http:// if scheme is missing
    let target_url =
        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("data:")
        {
            format!("http://{url}")
        } else {
            url.to_string()
        };

    let engine_used: String;
    let title: Option<String>;
    let mut status = Some(200);
    let console_errors: Vec<ConsoleError>;
    let network_errors: Vec<NetworkError>;
    let text_snippet: String;

    let browser_opt = find_system_browser();
    let node_available = has_node();

    let chosen_engine = match options.engine {
        WebInspectEngine::Auto => {
            if browser_opt.is_some() {
                WebInspectEngine::Cdp
            } else {
                WebInspectEngine::Http
            }
        }
        other => other,
    };

    match chosen_engine {
        WebInspectEngine::Cdp | WebInspectEngine::Playwright | WebInspectEngine::Puppeteer => {
            if node_available && browser_opt.is_some() {
                let browser_path = browser_opt.unwrap();
                engine_used = format!(
                    "cdp-{}",
                    browser_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("browser")
                );
                let res = run_node_cdp_inspector(&browser_path, &target_url, options.wait_ms)?;
                title = res.title;
                console_errors = res.console_errors;
                network_errors = res.network_errors;
                text_snippet = res.text_snippet;
            } else if let Some(browser_path) = browser_opt {
                engine_used = format!(
                    "browser-dump-{}",
                    browser_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("browser")
                );
                let res = run_direct_browser_dump(&browser_path, &target_url)?;
                title = res.title;
                console_errors = Vec::new();
                network_errors = Vec::new();
                text_snippet = res.text_snippet;
            } else {
                engine_used = "http-fallback".to_string();
                let res = run_http_probe(&target_url)?;
                status = res.status;
                title = res.title;
                console_errors = Vec::new();
                network_errors = Vec::new();
                text_snippet = res.text_snippet;
            }
        }
        WebInspectEngine::Http => {
            engine_used = "http-probe".to_string();
            let res = run_http_probe(&target_url)?;
            status = res.status;
            title = res.title;
            console_errors = Vec::new();
            network_errors = Vec::new();
            text_snippet = res.text_snippet;
        }
        WebInspectEngine::Auto => unreachable!(),
    }

    // Correlate console & runtime errors with workspace source files
    let mut correlated_files = Vec::new();
    if options.correlate_workspace {
        if let Some(ws) = workspace {
            correlated_files = correlate_errors_to_workspace(&console_errors, ws);
        }
    }

    // Generate human-readable diagnostic summary
    let summary = format_diagnostic_summary(
        &target_url,
        title.as_deref(),
        status,
        &engine_used,
        &console_errors,
        &network_errors,
        &correlated_files,
    );

    Ok(WebInspectReport {
        url: target_url,
        title,
        status,
        engine_used,
        console_errors,
        network_errors,
        text_snippet,
        correlated_files,
        summary,
    })
}

#[derive(Default, Deserialize)]
struct NodeCdpOutput {
    title: Option<String>,
    #[serde(default)]
    console_errors: Vec<ConsoleError>,
    #[serde(default)]
    network_errors: Vec<NetworkError>,
    #[serde(default)]
    text_snippet: String,
}

fn run_node_cdp_inspector(
    browser_path: &Path,
    target_url: &str,
    wait_ms: u64,
) -> anyhow::Result<NodeCdpOutput> {
    let script = r#"
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

async function main() {
  const args = process.argv.slice(1).filter(a => a !== '[eval]' && !a.endsWith('node.exe') && !a.endsWith('node'));
  const browserPath = args[0];
  const targetUrl = args[1];
  const waitMs = parseInt(args[2] || '1500', 10);

  const tmpUserData = path.join(os.tmpdir(), 'helpofai-inspect-' + Date.now() + '-' + Math.floor(Math.random() * 10000));
  fs.mkdirSync(tmpUserData, { recursive: true });

  const port = 9300 + Math.floor(Math.random() * 600);

  const proc = spawn(browserPath, [
    '--headless=new',
    '--disable-gpu',
    '--no-sandbox',
    '--disable-extensions',
    `--remote-debugging-port=${port}`,
    `--user-data-dir=${tmpUserData}`,
    'about:blank'
  ], { stdio: 'ignore' });

  proc.on('error', (err) => {
    try { fs.rmSync(tmpUserData, { recursive: true, force: true }); } catch (e) {}
    process.stdout.write(JSON.stringify({
      title: null,
      console_errors: [{ type: 'browser_spawn_error', message: err.message }],
      network_errors: [],
      text_snippet: ''
    }));
  });

  let wsUrl = null;
  for (let i = 0; i < 35; i++) {
    await new Promise(r => setTimeout(r, 100));
    try {
      const res = await fetch(`http://127.0.0.1:${port}/json/version`);
      if (res.ok) {
        const data = await res.json();
        wsUrl = data.webSocketDebuggerUrl;
        break;
      }
    } catch (e) {}
  }

  if (!wsUrl) {
    try { proc.kill(); } catch (e) {}
    try { fs.rmSync(tmpUserData, { recursive: true, force: true }); } catch (e) {}
    process.stdout.write(JSON.stringify({ title: null, console_errors: [{ type: 'browser_launch_error', message: 'Failed to connect to browser CDP port' }], network_errors: [], text_snippet: '' }));
    return;
  }

  let pageWsUrl = null;
  try {
    const newPageRes = await fetch(`http://127.0.0.1:${port}/json/new?${encodeURIComponent(targetUrl)}`, { method: 'PUT' });
    const pageData = await newPageRes.json();
    pageWsUrl = pageData.webSocketDebuggerUrl;
  } catch (e) {
    try { proc.kill(); } catch (e) {}
    try { fs.rmSync(tmpUserData, { recursive: true, force: true }); } catch (e) {}
    process.stdout.write(JSON.stringify({ title: null, console_errors: [{ type: 'navigation_error', message: 'Failed to create browser target: ' + e.message }], network_errors: [], text_snippet: '' }));
    return;
  }

  const ws = new WebSocket(pageWsUrl);

  const console_errors = [];
  const network_errors = [];
  let title = null;
  let text_snippet = '';

  await new Promise((resolve) => {
    let msgId = 1;
    const send = (method, params = {}) => {
      try {
        ws.send(JSON.stringify({ id: msgId++, method, params }));
      } catch (e) {}
    };

    ws.onopen = () => {
      send('Runtime.enable');
      send('Page.enable');
      send('Network.enable');
      send('Log.enable');
    };

    ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data);
        if (msg.method === 'Runtime.consoleAPICalled') {
          const type = msg.params.type;
          const text = (msg.params.args || []).map(a => a.value || a.description || '').join(' ');
          if (type === 'error' || type === 'warn') {
            const frame = msg.params.stackTrace?.callFrames?.[0];
            console_errors.push({
              type: `console_${type}`,
              message: text,
              line: frame ? frame.lineNumber : undefined,
              column: frame ? frame.columnNumber : undefined,
              source: frame ? frame.url : undefined
            });
          }
        } else if (msg.method === 'Runtime.exceptionThrown') {
          const details = msg.params.exceptionDetails;
          const frame = details.stackTrace?.callFrames?.[0];
          console_errors.push({
            type: 'uncaught_exception',
            message: details.text + (details.exception?.description ? ': ' + details.exception.description : ''),
            line: frame ? frame.lineNumber : details.lineNumber,
            column: frame ? frame.columnNumber : details.columnNumber,
            source: frame ? frame.url : details.url,
            stack: details.stackTrace ? details.stackTrace.callFrames.map(f => `${f.functionName || '<anonymous>'} (${f.url}:${f.lineNumber}:${f.columnNumber})`).join('\n') : undefined
          });
        } else if (msg.method === 'Network.responseReceived') {
          const resp = msg.params.response;
          if (resp && resp.status >= 400) {
            network_errors.push({
              url: resp.url,
              status: resp.status,
              error_text: resp.statusText || `HTTP ${resp.status}`
            });
          }
        } else if (msg.method === 'Network.loadingFailed') {
          network_errors.push({
            url: msg.params.requestId || 'unknown',
            error_text: msg.params.errorText || msg.params.blockedReason || 'Request Failed'
          });
        }

        if (msg.id === 5 && msg.result?.result?.value) {
          title = msg.result.result.value;
        }
        if (msg.id === 6 && msg.result?.result?.value) {
          text_snippet = msg.result.result.value;
        }
      } catch (e) {}
    };

    setTimeout(() => {
      send('Runtime.evaluate', { expression: 'document.title' });
      send('Runtime.evaluate', { expression: 'document.body ? document.body.innerText.slice(0, 1000) : ""' });
      setTimeout(() => resolve(), 400);
    }, waitMs);
  });

  try { ws.close(); } catch (e) {}
  try { proc.kill(); } catch (e) {}
  try { fs.rmSync(tmpUserData, { recursive: true, force: true }); } catch (e) {}

  process.stdout.write(JSON.stringify({
    title,
    console_errors,
    network_errors,
    text_snippet
  }));
}

main().catch(err => {
  process.stdout.write(JSON.stringify({
    title: null,
    console_errors: [{ type: 'inspector_exception', message: err.message }],
    network_errors: [],
    text_snippet: ''
  }));
});
"#;

    let output = Command::new("node")
        .arg("-e")
        .arg(script)
        .arg(browser_path.to_string_lossy().as_ref())
        .arg(target_url)
        .arg(wait_ms.to_string())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        anyhow::bail!("Node CDP runner produced empty output. Stderr: {stderr}");
    }

    let parsed: NodeCdpOutput = serde_json::from_str(&stdout)?;
    Ok(parsed)
}

struct DirectBrowserDumpOutput {
    title: Option<String>,
    text_snippet: String,
}

fn run_direct_browser_dump(
    browser_path: &Path,
    target_url: &str,
) -> anyhow::Result<DirectBrowserDumpOutput> {
    let output = Command::new(browser_path)
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--no-sandbox")
        .arg("--dump-dom")
        .arg(target_url)
        .output()?;

    let html = String::from_utf8_lossy(&output.stdout).to_string();
    let title = extract_html_title(&html);
    let text_snippet = extract_visible_text(&html);

    Ok(DirectBrowserDumpOutput {
        title,
        text_snippet,
    })
}

struct HttpProbeOutput {
    status: Option<u16>,
    title: Option<String>,
    text_snippet: String,
}

fn run_http_probe(target_url: &str) -> anyhow::Result<HttpProbeOutput> {
    // If curl is available, use curl; otherwise fallback to basic connection
    let output = Command::new("curl")
        .arg("-s")
        .arg("-i")
        .arg("-L")
        .arg("--max-time")
        .arg("5")
        .arg(target_url)
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        let status = parse_http_status(&text);
        let title = extract_html_title(&text);
        let text_snippet = extract_visible_text(&text);
        return Ok(HttpProbeOutput {
            status,
            title,
            text_snippet,
        });
    }

    Ok(HttpProbeOutput {
        status: None,
        title: None,
        text_snippet: String::new(),
    })
}

fn parse_http_status(response_text: &str) -> Option<u16> {
    for line in response_text.lines() {
        if line.starts_with("HTTP/") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(code) = parts[1].parse::<u16>() {
                    return Some(code);
                }
            }
        }
    }
    None
}

fn extract_html_title(html: &str) -> Option<String> {
    let lower = html.to_lowercase();
    if let Some(start) = lower.find("<title>") {
        if let Some(end) = lower[start..].find("</title>") {
            let title_content = &html[start + 7..start + end];
            return Some(title_content.trim().to_string());
        }
    }
    None
}

fn extract_visible_text(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    let mut in_script_or_style = false;
    let lower = html.to_lowercase();

    let bytes = html.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            in_tag = true;
            if lower[i..].starts_with("<script") || lower[i..].starts_with("<style") {
                in_script_or_style = true;
            }
            if lower[i..].starts_with("</script") || lower[i..].starts_with("</style") {
                in_script_or_style = false;
            }
        } else if bytes[i] == b'>' {
            in_tag = false;
            text.push(' ');
        } else if !in_tag && !in_script_or_style {
            text.push(bytes[i] as char);
        }
        i += 1;
    }

    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Correlates runtime errors (from stack traces or source filenames) to actual source files
/// in the local workspace, extracting the lines around the failure.
pub fn correlate_errors_to_workspace(
    errors: &[ConsoleError],
    workspace: &Path,
) -> Vec<FileDiagnosticMatch> {
    let mut matches = Vec::new();

    for err in errors {
        // Look for candidate filename in source or stack
        let candidate_filename = extract_candidate_filename(err.source.as_deref())
            .or_else(|| extract_candidate_from_stack(err.stack.as_deref()))
            .or_else(|| extract_candidate_from_message(&err.message));

        let Some(filename) = candidate_filename else {
            continue;
        };

        // Locate file in workspace
        if let Some(found_file) = find_file_in_workspace(workspace, &filename) {
            let line_num = err.line.unwrap_or(1);
            let snippet = extract_file_snippet(&found_file, line_num);
            let suggested_fix = suggest_fix_for_error(&err.message, &snippet);

            let rel_path = found_file
                .strip_prefix(workspace)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| found_file.to_string_lossy().to_string());

            matches.push(FileDiagnosticMatch {
                file_path: rel_path,
                line_number: line_num,
                error_message: err.message.clone(),
                snippet,
                suggested_fix,
            });
        }
    }

    matches
}

fn extract_candidate_filename(source: Option<&str>) -> Option<String> {
    let s = source?;
    if s.is_empty() || s.starts_with("data:") {
        return None;
    }

    let url_clean = s
        .split('?')
        .next()
        .unwrap_or(s)
        .split('#')
        .next()
        .unwrap_or(s);
    let last_segment = url_clean.rsplit(['/', '\\']).next()?;

    if is_supported_source_ext(last_segment) {
        Some(last_segment.to_string())
    } else {
        None
    }
}

fn extract_candidate_from_stack(stack: Option<&str>) -> Option<String> {
    let s = stack?;
    for part in s.split(['(', ')', ' ', '\n']) {
        if let Some(fname) = extract_candidate_filename(Some(part.trim())) {
            return Some(fname);
        }
    }
    None
}

fn extract_candidate_from_message(msg: &str) -> Option<String> {
    for word in msg.split_whitespace() {
        let clean =
            word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-');
        if is_supported_source_ext(clean) {
            return Some(clean.to_string());
        }
    }
    None
}

fn is_supported_source_ext(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.ends_with(".js")
        || lower.ends_with(".jsx")
        || lower.ends_with(".ts")
        || lower.ends_with(".tsx")
        || lower.ends_with(".vue")
        || lower.ends_with(".svelte")
        || lower.ends_with(".html")
        || lower.ends_with(".css")
}

fn find_file_in_workspace(workspace: &Path, target_filename: &str) -> Option<PathBuf> {
    if !workspace.exists() {
        return None;
    }

    for entry in walkdir::WalkDir::new(workspace)
        .max_depth(8)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.')
                && name != "node_modules"
                && name != "vendor"
                && name != "storage"
                && name != "target"
                && name != "dist"
                && name != "build"
        })
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() && entry.file_name().to_string_lossy() == target_filename {
            return Some(entry.path().to_path_buf());
        }
    }

    None
}

fn extract_file_snippet(file_path: &Path, line_num: u32) -> String {
    let Ok(content) = std::fs::read_to_string(file_path) else {
        return String::from("(Could not read source file)");
    };

    let lines: Vec<&str> = content.lines().collect();
    let target = line_num as usize;
    let start = target.saturating_sub(3).max(1);
    let end = (target + 3).min(lines.len());

    let mut snippet = String::new();
    for i in start..=end {
        if i <= lines.len() {
            let marker = if i == target { ">>" } else { "  " };
            snippet.push_str(&format!("{} {:4} | {}\n", marker, i, lines[i - 1]));
        }
    }

    snippet
}

fn suggest_fix_for_error(err_message: &str, snippet: &str) -> String {
    if err_message.contains("Cannot read properties of undefined")
        || err_message.contains("undefined is not an object")
    {
        "Add optional chaining `?.` or default fallback `|| []` / `|| {}` before accessing the property.".to_string()
    } else if err_message.contains("is not defined") {
        "Verify imports or ensure variable is declared before usage.".to_string()
    } else if err_message.contains("Failed to fetch") || err_message.contains("NetworkError") {
        "Verify backend API server is running on the expected port and CORS headers (Access-Control-Allow-Origin) are enabled.".to_string()
    } else if snippet.contains("map(") {
        "Ensure array is initialized before calling `.map()`, e.g. `(items || []).map(...)`."
            .to_string()
    } else {
        "Inspect the indicated line and add proper validation or null-checks.".to_string()
    }
}

fn format_diagnostic_summary(
    url: &str,
    title: Option<&str>,
    status: Option<u16>,
    engine: &str,
    console_errors: &[ConsoleError],
    network_errors: &[NetworkError],
    correlated: &[FileDiagnosticMatch],
) -> String {
    let mut out = String::new();
    out.push_str(&format!("Web Page Inspection: {url}\n"));
    if let Some(t) = title {
        out.push_str(&format!("Title: {t}\n"));
    }
    if let Some(s) = status {
        out.push_str(&format!("HTTP Status: {s}\n"));
    }
    out.push_str(&format!("Engine: {engine}\n"));
    out.push_str(&format!(
        "Results: {} console errors/warnings, {} network errors, {} workspace file matches.\n\n",
        console_errors.len(),
        network_errors.len(),
        correlated.len()
    ));

    if !correlated.is_empty() {
        out.push_str("=== Correlated Workspace Errors (Ready For Auto-Fix) ===\n");
        for (idx, m) in correlated.iter().enumerate() {
            out.push_str(&format!(
                "[{}] File: {} (Line {})\nError: {}\nCode Snippet:\n{}\nSuggested Fix: {}\n\n",
                idx + 1,
                m.file_path,
                m.line_number,
                m.error_message,
                m.snippet,
                m.suggested_fix
            ));
        }
    }

    if !console_errors.is_empty() && correlated.is_empty() {
        out.push_str("=== Console Errors & Warnings ===\n");
        for err in console_errors {
            out.push_str(&format!(
                "- [{}] {}{}\n",
                err.error_type,
                err.message,
                err.source
                    .as_ref()
                    .map(|s| format!(" ({s})"))
                    .unwrap_or_default()
            ));
        }
        out.push('\n');
    }

    if !network_errors.is_empty() {
        out.push_str("=== Network Failures ===\n");
        for net in network_errors {
            out.push_str(&format!(
                "- {} -> {}\n",
                net.url,
                net.status
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| net.error_text.clone())
            ));
        }
        out.push('\n');
    }

    if console_errors.is_empty() && network_errors.is_empty() {
        out.push_str(
            "No JavaScript errors, console warnings, or failed network requests detected.\n",
        );
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_html_title() {
        let html = "<html><head><title>My Test App</title></head><body>Hello</body></html>";
        assert_eq!(extract_html_title(html), Some("My Test App".to_string()));
    }

    #[test]
    fn test_extract_visible_text() {
        let html = "<html><body><h1>Hello World</h1><script>console.log('hi');</script><p>This is content.</p></body></html>";
        let text = extract_visible_text(html);
        assert!(text.contains("Hello World"));
        assert!(text.contains("This is content."));
        assert!(!text.contains("console.log"));
    }

    #[test]
    fn test_suggest_fix_for_null_error() {
        let err = "Cannot read properties of undefined (reading 'map')";
        let snippet = "24 | items.map(x => x.id)";
        let fix = suggest_fix_for_error(err, snippet);
        assert!(fix.contains("optional chaining") || fix.contains("fallback"));
    }

    #[test]
    fn test_correlate_errors_to_workspace() {
        let temp = tempfile::tempdir().unwrap();
        let src_dir = temp.path().join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        let test_file = src_dir.join("App.jsx");
        std::fs::write(&test_file, "import React from 'react';\n\nexport function App() {\n  const data = undefined;\n  return <div>{data.name}</div>;\n}\n").unwrap();

        let errors = vec![ConsoleError {
            error_type: "uncaught_exception".to_string(),
            message: "Cannot read properties of undefined (reading 'name')".to_string(),
            line: Some(5),
            column: Some(18),
            source: Some("http://localhost:3000/src/App.jsx".to_string()),
            stack: None,
        }];

        let matches = correlate_errors_to_workspace(&errors, temp.path());
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line_number, 5);
        assert!(matches[0].file_path.contains("App.jsx"));
        assert!(
            matches[0]
                .snippet
                .contains("return <div>{data.name}</div>;")
        );
    }
}
