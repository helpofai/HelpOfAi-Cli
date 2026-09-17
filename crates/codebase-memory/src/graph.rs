/*
 * Copyright (c) 2026 HelpOfAi. All rights reserved.
 *
 * This file is part of the HelpOfAi CLI codebase.
 * It is subject to the MIT license terms in the LICENSE.md file
 * found in the top-level directory of this distribution.
 */

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use tracing::info;

use crate::platform::{engine_binary_path, engine_dir};

// ── PID file path ─────────────────────────────────────────────────────────────

fn pid_file_path() -> Result<PathBuf> {
    Ok(engine_dir()?.join("runtime").join("graph.pid"))
}

fn port_file_path() -> Result<PathBuf> {
    Ok(engine_dir()?.join("runtime").join("graph.port"))
}

// ── GraphSupervisor ───────────────────────────────────────────────────────────

/// Spawns and manages the `codebase-memory-mcp --ui=true` process.
pub struct GraphSupervisor {
    pub binary: PathBuf,
}

impl GraphSupervisor {
    pub fn new() -> Result<Self> {
        let binary = engine_binary_path()?;
        if !binary.exists() {
            println!(
                "Codebase Memory engine not found. Automatically downloading and installing..."
            );
            let installer = crate::installer::Installer::new()?;
            installer.install(false)?;
        }
        Ok(Self { binary })
    }

    pub fn new_silent() -> Result<Self> {
        let binary = engine_binary_path()?;
        if !binary.exists() {
            let installer = crate::installer::Installer::new_silent()?;
            installer.install(false)?;
        }
        Ok(Self { binary })
    }

    /// Check if the HTTP server on port is accepting TCP connections.
    pub fn is_port_responding(port: u16) -> bool {
        is_port_responding(port)
    }

    /// Synchronously poll until `http://localhost:<port>/` responds or timeout expires.
    pub fn wait_until_ready_sync(&self, port: u16, timeout: Duration) -> bool {
        let start = std::time::Instant::now();
        while start.elapsed() < timeout {
            if Self::is_port_responding(port) {
                return true;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        false
    }

    /// Ensure the engine is running in the background and open the browser to the graph UI.
    /// If the server is already running, simply opens the browser to the existing port.
    pub fn ensure_running_and_open_browser(port: u16) -> Result<()> {
        if Self::is_port_responding(port) {
            let active_port = read_port().unwrap_or(port);
            open_browser(active_port)?;
            return Ok(());
        }

        let supervisor = Self::new_silent()?;
        let _ = supervisor.start_daemon(port)?;

        if supervisor.wait_until_ready_sync(port, Duration::from_secs(10)) {
            let _ = open_browser(port);
        } else {
            tracing::warn!("Codebase Memory daemon did not respond on port {port} within 10s");
        }

        Ok(())
    }

    /// Start the engine as a permanent background daemon with HTTP UI on `port`.
    pub fn start_daemon(&self, port: u16) -> Result<Option<u32>> {
        let engine_root = engine_dir()?;
        let cache_dir = engine_root.join("cache");
        let runtime_dir = engine_root.join("runtime");
        let logs_dir = engine_root.join("logs");
        std::fs::create_dir_all(&cache_dir).ok();
        std::fs::create_dir_all(&runtime_dir).ok();
        std::fs::create_dir_all(&logs_dir).ok();

        let mut cmd = Command::new(&self.binary);
        cmd.arg("daemon").arg("start").arg(format!("--port={port}"));
        cmd.env("CBM_CACHE_DIR", &cache_dir);
        cmd.env("CBM_RUNTIME_DIR", &runtime_dir);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let output = cmd
            .output()
            .context("Failed to execute `codebase-memory-mcp daemon start`")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let pid = parse_pid_from_output(&stdout).or_else(|| parse_pid_from_output(&stderr));

        if let Some(p) = pid {
            let _ = write_pid(p, port);
        } else if let Ok(path) = port_file_path() {
            let _ = std::fs::write(&path, port.to_string());
        }

        Ok(pid)
    }

    /// Start the graph UI server in the background and return its PID if detected.
    pub fn start(&self, port: u16) -> Result<Option<u32>> {
        self.start_daemon(port)
    }

    /// Poll `http://127.0.0.1:<port>/` until it responds or we time out (~10 s).
    pub async fn wait_until_ready(&self, port: u16) -> Result<()> {
        let url = format!("http://localhost:{port}/");
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .build()?;

        for _ in 0..50u32 {
            if client.get(&url).send().await.is_ok() {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        bail!(
            "Codebase Memory graph server did not become ready at {url} within 10 seconds.\n\
             Check `helpofai graph status` or the engine logs."
        )
    }
}

// ── PID helpers ───────────────────────────────────────────────────────────────

fn write_pid(pid: u32, port: u16) -> Result<()> {
    let pf = pid_file_path()?;
    let portf = port_file_path()?;
    std::fs::write(&pf, pid.to_string())?;
    std::fs::write(&portf, port.to_string())?;
    info!("Wrote PID {} to {}", pid, pf.display());
    Ok(())
}

pub(crate) fn read_pid() -> Option<u32> {
    pid_file_path()
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| s.trim().parse().ok())
}

pub(crate) fn read_port() -> Option<u16> {
    port_file_path()
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| s.trim().parse().ok())
}

/// Check if the HTTP server on port is accepting TCP connections.
pub fn is_port_responding(port: u16) -> bool {
    use std::net::{SocketAddr, TcpStream};
    TcpStream::connect_timeout(
        &SocketAddr::from(([127, 0, 0, 1], port)),
        Duration::from_millis(150),
    )
    .is_ok()
}

/// True if the graph server is responding on its configured port.
pub fn is_running() -> bool {
    let port = read_port().unwrap_or(9749);
    is_port_responding(port)
}

/// Parse PID from engine daemon output (e.g. "pid 12345" or "pid: 12345").
pub(crate) fn parse_pid_from_output(text: &str) -> Option<u32> {
    for line in text.lines() {
        if let Some(idx) = line.find("pid") {
            let rest = line[idx + 3..].trim_start_matches(|c: char| c == ':' || c.is_whitespace());
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(pid) = digits.parse::<u32>() {
                return Some(pid);
            }
        }
    }
    None
}

/// Print a human-readable status line to stdout.
pub fn print_status() {
    let port = read_port().unwrap_or(9749);
    let mut pid = read_pid();

    // If port is responding but we don't have PID recorded, try to detect via `daemon status`
    if is_port_responding(port) && pid.is_none() {
        if let Ok(binary) = engine_binary_path() {
            if binary.exists() {
                let mut cmd = Command::new(&binary);
                cmd.args(["daemon", "status"]);
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                    cmd.creation_flags(CREATE_NO_WINDOW);
                }
                if let Ok(out) = cmd.output() {
                    let s = String::from_utf8_lossy(&out.stdout);
                    if let Some(p) = parse_pid_from_output(&s) {
                        pid = Some(p);
                        let _ = write_pid(p, port);
                    }
                }
            }
        }
    }

    if is_port_responding(port) {
        if let Some(p) = pid {
            println!(
                "Graph server: running (PID {p}, port {port})\n\
                 URL: http://localhost:{port}"
            );
        } else {
            println!(
                "Graph server: running (port {port})\n\
                 URL: http://localhost:{port}"
            );
        }
    } else if let Some(p) = pid {
        println!("Graph server: stopped (stale PID {p})");
        pid_file_path()
            .ok()
            .and_then(|p| std::fs::remove_file(p).ok());
        port_file_path()
            .ok()
            .and_then(|p| std::fs::remove_file(p).ok());
    } else {
        println!("Graph server: not running");
    }
}

/// Stop the background daemon and remove PID/port files.
pub fn stop_by_pid() -> Result<()> {
    let mut stopped_any = false;

    // Ask daemon cleanly to stop via the binary
    if let Ok(binary) = engine_binary_path() {
        if binary.exists() {
            let mut cmd = Command::new(&binary);
            cmd.args(["daemon", "stop"]);
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                cmd.creation_flags(CREATE_NO_WINDOW);
            }
            if let Ok(out) = cmd.output() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if stdout.contains("stopping") {
                    stopped_any = true;
                }
            }
        }
    }

    // If tracked PID is still alive, kill it
    if let Some(pid) = read_pid() {
        if pid_is_alive(pid) {
            let _ = kill_pid(pid);
            stopped_any = true;
        }
    }

    pid_file_path()
        .ok()
        .and_then(|p| std::fs::remove_file(p).ok());
    port_file_path()
        .ok()
        .and_then(|p| std::fs::remove_file(p).ok());

    if stopped_any {
        println!("Graph server stopped.");
    } else {
        println!("Graph server is not running.");
    }
    Ok(())
}

// ── OS-specific process utilities ─────────────────────────────────────────────

#[cfg(target_os = "windows")]
pub(crate) fn pid_is_alive(pid: u32) -> bool {
    // On Windows: check if the process exists via tasklist
    let out = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/NH", "/FO", "CSV"])
        .output();
    match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            s.contains(&pid.to_string())
        }
        Err(_) => false,
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn pid_is_alive(pid: u32) -> bool {
    // POSIX: kill -0 sends no signal but checks existence / permission
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn kill_pid(pid: u32) -> Result<()> {
    let status = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .status()?;
    if !status.success() {
        bail!("taskkill failed for PID {pid}");
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn kill_pid(pid: u32) -> Result<()> {
    let status = Command::new("kill")
        .args(["-TERM", &pid.to_string()])
        .status()?;
    if !status.success() {
        bail!("kill -TERM failed for PID {pid}");
    }
    Ok(())
}

// ── Browser ───────────────────────────────────────────────────────────────────

/// Open the user's default browser to the graph UI URL.
pub fn open_browser(port: u16) -> Result<()> {
    let url = format!("http://localhost:{port}");
    println!("Opening browser: {url}");

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &url])
            .spawn()
            .context("Failed to open browser")?
            .wait()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .context("Failed to open browser")?
            .wait()?;
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .context("Failed to open browser (is xdg-open installed?)")?
            .wait()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pid_from_output() {
        assert_eq!(
            parse_pid_from_output("daemon: started (permanent, pid 48880)"),
            Some(48880)
        );
        assert_eq!(
            parse_pid_from_output("  pid: 12345\n  build: abc"),
            Some(12345)
        );
        assert_eq!(
            parse_pid_from_output("daemon: already active (permanent, pid 99999)"),
            Some(99999)
        );
        assert_eq!(parse_pid_from_output("daemon: not running"), None);
    }
}
