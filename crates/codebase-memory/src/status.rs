/*
 * Copyright (c) 2026 HelpOfAi. All rights reserved.
 *
 * This file is part of the HelpOfAi CLI codebase.
 * It is subject to the MIT license terms in the LICENSE.md file
 * found in the top-level directory of this distribution.
 */

use std::fmt;
use std::path::PathBuf;

use crate::graph::{pid_is_alive, read_pid, read_port};
use crate::platform::{engine_binary_path, engine_dir};

fn download_lock_path() -> Option<PathBuf> {
    engine_dir()
        .ok()
        .map(|d| d.join("runtime").join("downloading.lock"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodebaseMemoryStatus {
    /// Engine is actively downloading/updating.
    Downloading,
    /// Engine is running and responding (PID and port tracked).
    Running { port: u16, pid: u32 },
    /// Engine binary is installed, but the graph service is stopped.
    Stopped,
    /// Engine binary has not been installed yet.
    NotInstalled,
}

impl CodebaseMemoryStatus {
    #[must_use]
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }

    #[must_use]
    pub fn port(&self) -> Option<u16> {
        match self {
            Self::Running { port, .. } => Some(*port),
            _ => None,
        }
    }

    /// Status badge icon and label.
    #[must_use]
    pub fn badge(&self) -> (&'static str, &'static str) {
        match self {
            Self::Running { .. } => ("●", "Running"),
            Self::Downloading => ("⟳", "Downloading"),
            Self::Stopped => ("○", "Stopped"),
            Self::NotInstalled => ("✕", "Not Installed"),
        }
    }

    /// Header chip text representation for compact UI bars.
    #[must_use]
    pub fn chip_text(&self) -> String {
        match self {
            Self::Running { port, .. } => format!("🧠 ● {port}"),
            Self::Downloading => "🧠 ⟳ dl...".to_string(),
            Self::Stopped => "🧠 ○ off".to_string(),
            Self::NotInstalled => "🧠 ✕".to_string(),
        }
    }
}

impl fmt::Display for CodebaseMemoryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running { port, pid } => {
                write!(
                    f,
                    "Running (PID {pid}, Port {port}, http://localhost:{port})"
                )
            }
            Self::Downloading => write!(f, "Downloading / Installing"),
            Self::Stopped => write!(f, "Stopped"),
            Self::NotInstalled => write!(f, "Not Installed"),
        }
    }
}

/// Probes the local filesystem and process list to determine Codebase Memory engine status.
#[must_use]
pub fn probe_status() -> CodebaseMemoryStatus {
    // 1. Check if downloading lock exists and process is alive
    if let Some(lock) = download_lock_path() {
        if lock.exists() {
            if let Ok(content) = std::fs::read_to_string(&lock) {
                if let Ok(pid) = content.trim().parse::<u32>() {
                    if pid_is_alive(pid) {
                        return CodebaseMemoryStatus::Downloading;
                    }
                }
            }
            // Lock was stale, clean up
            let _ = std::fs::remove_file(lock);
        }
    }

    // 2. Check if binary exists
    let Ok(binary) = engine_binary_path() else {
        return CodebaseMemoryStatus::NotInstalled;
    };
    if !binary.exists() {
        return CodebaseMemoryStatus::NotInstalled;
    }

    // 3. Check if running
    let port = read_port().unwrap_or(9749);
    if crate::graph::is_port_responding(port) {
        let pid = read_pid().unwrap_or(0);
        return CodebaseMemoryStatus::Running { port, pid };
    }

    CodebaseMemoryStatus::Stopped
}
