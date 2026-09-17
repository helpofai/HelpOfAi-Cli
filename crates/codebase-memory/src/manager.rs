/*
 * Copyright (c) 2026 HelpOfAi. All rights reserved.
 *
 * This file is part of the HelpOfAi CLI codebase.
 * It is subject to the MIT license terms in the LICENSE.md file
 * found in the top-level directory of this distribution.
 */

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Result, bail};
use tracing::{info, warn};

use crate::platform::engine_binary_path;

pub struct CodebaseMemoryManager {
    binary: PathBuf,
}

impl CodebaseMemoryManager {
    pub fn new() -> Result<Self> {
        let binary = engine_binary_path()?;
        Ok(Self { binary })
    }

    pub fn is_installed(&self) -> bool {
        self.binary.exists()
    }

    /// Automatically download and install the engine if not present.
    pub fn ensure_installed(&self) -> Result<()> {
        if !self.is_installed() {
            println!(
                "Codebase Memory engine not found. Automatically downloading and installing..."
            );
            let installer = crate::installer::Installer::new()?;
            installer.install(false)?;
        }
        Ok(())
    }

    /// Run the `daemon stop` command provided natively by the upstream binary.
    /// This sends IPC to the coordination daemon to shut down gracefully.
    pub fn stop_daemon(&self) -> Result<()> {
        if !self.is_installed() {
            return Ok(());
        }
        info!("Stopping Codebase Memory daemon...");
        let status = Command::new(&self.binary)
            .arg("daemon")
            .arg("stop")
            .env(
                "CBM_CACHE_DIR",
                crate::platform::engine_dir()?.join("cache"),
            )
            .env(
                "CBM_RUNTIME_DIR",
                crate::platform::engine_dir()?.join("runtime"),
            )
            .status()?;

        if !status.success() {
            warn!(
                "daemon stop returned non-zero exit status: {:?}",
                status.code()
            );
        }
        Ok(())
    }

    /// Run the `daemon status` command provided natively by the upstream binary.
    pub fn get_status(&self) -> Result<()> {
        if !self.is_installed() {
            self.ensure_installed()?;
        }
        let status = Command::new(&self.binary)
            .arg("daemon")
            .arg("status")
            .env(
                "CBM_CACHE_DIR",
                crate::platform::engine_dir()?.join("cache"),
            )
            .env(
                "CBM_RUNTIME_DIR",
                crate::platform::engine_dir()?.join("runtime"),
            )
            .status()?;

        if !status.success() {
            warn!(
                "daemon status returned non-zero exit status: {:?}",
                status.code()
            );
        }
        Ok(())
    }

    /// Pass a raw codebase command (index, search, etc.) through to the binary.
    pub fn run_passthrough(&self, args: &[String]) -> Result<()> {
        if !self.is_installed() {
            self.ensure_installed()?;
        }
        let status = Command::new(&self.binary)
            .args(args)
            .env(
                "CBM_CACHE_DIR",
                crate::platform::engine_dir()?.join("cache"),
            )
            .env(
                "CBM_RUNTIME_DIR",
                crate::platform::engine_dir()?.join("runtime"),
            )
            .status()?;

        if !status.success() {
            bail!("Command failed with exit code {:?}", status.code());
        }
        Ok(())
    }

    /// Index a repository path into the semantic knowledge graph.
    pub fn index_repository(&self, repo_path: &str) -> Result<()> {
        let abs_path = std::path::Path::new(repo_path)
            .canonicalize()
            .unwrap_or_else(|_| std::path::PathBuf::from(repo_path));
        let args_json = serde_json::json!({
            "repo_path": abs_path.to_string_lossy(),
        })
        .to_string();
        self.run_passthrough(&["cli".to_string(), "index_repository".to_string(), args_json])
    }

    /// Search the codebase graph using a regex pattern.
    pub fn search_graph(&self, pattern: &str) -> Result<()> {
        let args_json = serde_json::json!({
            "name_pattern": pattern,
        })
        .to_string();
        self.run_passthrough(&["cli".to_string(), "search_graph".to_string(), args_json])
    }

    /// Run the upstream agent auto-configuration installer (`codebase-memory-mcp install`).
    pub fn setup_agents(&self) -> Result<()> {
        self.run_passthrough(&["install".to_string()])
    }

    /// Run configuration commands against the engine's config store.
    pub fn run_config(&self, args: &[String]) -> Result<()> {
        let mut full_args = vec!["config".to_string()];
        full_args.extend_from_slice(args);
        self.run_passthrough(&full_args)
    }
}
