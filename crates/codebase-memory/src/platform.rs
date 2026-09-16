/*
 * Copyright (c) 2026 HelpOfAi. All rights reserved.
 *
 * This file is part of the HelpOfAi CLI codebase.
 * It is subject to the MIT license terms in the LICENSE.md file
 * found in the top-level directory of this distribution.
 */

use std::path::PathBuf;

/// Get the canonical HelpOfAi data directory.
///
/// Windows: `%LOCALAPPDATA%\HelpOfAi`
/// macOS/Linux: `~/.local/share/helpofai`
pub fn helpofai_data_dir() -> anyhow::Result<PathBuf> {
    if cfg!(target_os = "windows") {
        let base = std::env::var("LOCALAPPDATA")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::data_local_dir)
            .ok_or_else(|| anyhow::anyhow!("Cannot determine %LOCALAPPDATA%"))?;
        Ok(base.join("HelpOfAi"))
    } else {
        let root = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".local").join("share"));
        Ok(root.join("helpofai"))
    }
}

/// Directory where the engine binary and its data live.
pub fn engine_dir() -> anyhow::Result<PathBuf> {
    Ok(helpofai_data_dir()?.join("engines").join("codebase-memory"))
}

/// Absolute path to the `codebase-memory-mcp` executable.
pub fn engine_binary_path() -> anyhow::Result<PathBuf> {
    let dir = engine_dir()?;
    if cfg!(target_os = "windows") {
        Ok(dir.join("helpofai-codebase-memory.exe"))
    } else {
        Ok(dir.join("helpofai-codebase-memory"))
    }
}

/// CPU architecture string used in GitHub release artifact names.
/// Returns an error at runtime for targets with no pre-built upstream binary.
pub fn get_architecture() -> anyhow::Result<&'static str> {
    #[cfg(target_arch = "x86_64")]
    return Ok("amd64");

    #[cfg(target_arch = "aarch64")]
    return Ok("arm64");

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    anyhow::bail!(
        "crates/codebase-memory: no pre-built codebase-memory-mcp binary for this \
         target architecture. Only x86_64 (amd64) and aarch64 (arm64) are supported."
    );
}

/// OS string used in GitHub release artifact names.
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    }
}

#[cfg(test)]
#[path = "platform_tests.rs"]
mod tests;
