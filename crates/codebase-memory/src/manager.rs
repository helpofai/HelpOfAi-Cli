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
            bail!("Codebase Memory engine is not installed.");
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
            bail!("Daemon is not running or encountered an error.");
        }
        Ok(())
    }

    /// Pass a raw codebase command (index, search, etc.) through to the binary.
    pub fn run_passthrough(&self, args: &[String]) -> Result<()> {
        if !self.is_installed() {
            bail!(
                "Codebase Memory engine is not installed. Run `helpofai codebase install` first."
            );
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
}
