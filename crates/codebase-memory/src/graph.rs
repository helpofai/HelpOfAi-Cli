use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
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
            bail!(
                "Codebase Memory engine is not installed at {}.\n\
                 Run `helpofai codebase install` first.",
                binary.display()
            );
        }
        Ok(Self { binary })
    }

    /// Spawn the graph UI process, write a PID file, and optionally poll for
    /// readiness before returning the child handle.
    pub fn start(&self, port: u16) -> Result<Child> {
        let engine_root = engine_dir()?;
        let cache_dir = engine_root.join("cache");
        let runtime_dir = engine_root.join("runtime");
        std::fs::create_dir_all(&cache_dir).ok();
        std::fs::create_dir_all(&runtime_dir).ok();

        let mut cmd = Command::new(&self.binary);
        cmd.arg("--ui=true");
        cmd.arg("--port").arg(port.to_string());
        cmd.env("CBM_CACHE_DIR", &cache_dir);
        cmd.env("CBM_RUNTIME_DIR", &runtime_dir);
        // pipe stdin so we can hold the other end open, preventing the child from seeing EOF immediately
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let child = cmd
            .spawn()
            .context("Failed to spawn Codebase Memory engine")?;

        // Persist PID and port so `stop` / `status` can find our process later.
        write_pid(child.id(), port)?;

        Ok(child)
    }

    /// Poll `http://127.0.0.1:<port>/` until it responds or we time out (~10 s).
    pub async fn wait_until_ready(&self, port: u16) -> Result<()> {
        let url = format!("http://localhost:{}/", port);
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
            "Codebase Memory graph server did not become ready at {} within 10 seconds.\n\
             Check `helpofai graph status` or the engine logs.",
            url
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

fn read_pid() -> Option<u32> {
    pid_file_path()
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| s.trim().parse().ok())
}

fn read_port() -> Option<u16> {
    port_file_path()
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| s.trim().parse().ok())
}

/// True if the PID from the pid-file is currently alive.
pub fn is_running() -> bool {
    match read_pid() {
        None => false,
        Some(pid) => pid_is_alive(pid),
    }
}

/// Print a human-readable status line to stdout.
pub fn print_status() {
    match read_pid() {
        None => println!("Graph server: not running (no PID file found)"),
        Some(pid) => {
            let port = read_port().unwrap_or(9749);
            if pid_is_alive(pid) {
                println!(
                    "Graph server: running (PID {pid}, port {port})\n\
                     URL: http://localhost:{port}"
                );
            } else {
                println!("Graph server: stopped (stale PID {pid})");
                // Clean up stale file
                pid_file_path().ok().map(|p| std::fs::remove_file(p).ok());
            }
        }
    }
}

/// Kill the process whose PID we tracked, then remove the pid-file.
pub fn stop_by_pid() -> Result<()> {
    match read_pid() {
        None => {
            println!("Graph server is not running (no PID file).");
            Ok(())
        }
        Some(pid) => {
            if pid_is_alive(pid) {
                kill_pid(pid)?;
                println!("Graph server (PID {pid}) stopped.");
            } else {
                println!("Graph server already stopped (stale PID {pid}).");
            }
            pid_file_path().ok().map(|p| std::fs::remove_file(p).ok());
            port_file_path().ok().map(|p| std::fs::remove_file(p).ok());
            Ok(())
        }
    }
}

// ── OS-specific process utilities ─────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn pid_is_alive(pid: u32) -> bool {
    // On Windows: check if the process exists via tasklist
    let out = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/NH", "/FO", "CSV"])
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
fn pid_is_alive(pid: u32) -> bool {
    // POSIX: kill -0 sends no signal but checks existence / permission
    use std::os::unix::process::ExitStatusExt;
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
        bail!("taskkill failed for PID {}", pid);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn kill_pid(pid: u32) -> Result<()> {
    let status = Command::new("kill")
        .args(["-TERM", &pid.to_string()])
        .status()?;
    if !status.success() {
        bail!("kill -TERM failed for PID {}", pid);
    }
    Ok(())
}

// ── Browser ───────────────────────────────────────────────────────────────────

/// Open the user's default browser to the graph UI URL.
pub fn open_browser(port: u16) -> Result<()> {
    let url = format!("http://localhost:{}", port);
    println!("Opening browser: {}", url);

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
