/*
 * Copyright (c) 2026 HelpOfAi. All rights reserved.
 *
 * This file is part of the HelpOfAi CLI codebase.
 * It is subject to the MIT license terms in the LICENSE.md file
 * found in the top-level directory of this distribution.
 */

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use anyhow::{Result, bail};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};

use crate::platform::{engine_binary_path, engine_dir, get_architecture, get_platform};

const BASE_URL: &str = "https://github.com/DeusData/codebase-memory-mcp/releases/latest/download";

pub struct Installer {
    client: Client,
}

impl Installer {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()?;
        Ok(Self { client })
    }

    // ── public API ────────────────────────────────────────────────────────────

    /// Install the engine. Skips if already installed and `force` is false.
    pub fn install(&self, force: bool) -> Result<()> {
        let binary = engine_binary_path()?;
        if binary.exists() && !force {
            let ver = probe_version(&binary)?;
            println!("Codebase Memory engine already installed ({ver})");
            println!("Run `helpofai codebase update` to upgrade.");
            return Ok(());
        }
        self.download_and_install()
    }

    /// Check for a newer version and replace the binary if one exists.
    pub fn update(&self) -> Result<()> {
        let binary = engine_binary_path()?;
        if binary.exists() {
            let existing = probe_version(&binary).unwrap_or_default();
            println!("Current version: {existing}");
        } else {
            println!("Engine not installed — performing fresh install.");
        }
        println!("Checking for latest release…");
        self.download_and_install()
    }

    // ── core download/install pipeline ────────────────────────────────────────

    fn download_and_install(&self) -> Result<()> {
        let platform = get_platform();
        let arch = get_architecture();
        let ext = if platform == "windows" {
            "zip"
        } else {
            "tar.gz"
        };
        let archive_name = format!("codebase-memory-mcp-{platform}-{arch}.{ext}");

        let temp_dir = tempfile::Builder::new().prefix("cbm-install-").tempdir()?;
        let temp_path = temp_dir.path();

        // 1 ── fetch checksums
        let checksums_url = format!("{BASE_URL}/checksums.txt");
        println!("Fetching checksums…");
        let checksums_txt = self.client.get(&checksums_url).send()?.text()?;
        let expected_hash = self.parse_checksum(&checksums_txt, &archive_name)?;

        // 2 ── streaming download with byte counter
        let archive_url = format!("{BASE_URL}/{archive_name}");
        println!("Downloading {archive_url}…");
        let mut response = self.client.get(&archive_url).send()?;
        if !response.status().is_success() {
            bail!("HTTP {} downloading archive", response.status());
        }
        let total = response.content_length();
        let archive_path = temp_path.join(&archive_name);
        {
            let mut dest = File::create(&archive_path)?;
            let mut downloaded: u64 = 0;
            let mut buf = [0u8; 65536];
            use std::io::Read;
            loop {
                let n = response.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                dest.write_all(&buf[..n])?;
                downloaded += n as u64;
                if let Some(total) = total {
                    let pct = downloaded * 100 / total;
                    eprint!("\r  {downloaded} / {total} bytes  ({pct}%)");
                } else {
                    eprint!("\r  {downloaded} bytes");
                }
            }
            eprintln!();
        }

        // 3 ── verify
        println!("Verifying SHA-256 checksum…");
        self.verify_file_hash(&archive_path, &expected_hash)?;
        println!("  Checksum OK.");

        // 4 ── extract safely (atomic replacement with rollback)
        let engine_dest_dir = engine_dir()?;
        let rollback_dir = engine_dest_dir.with_file_name("codebase-memory-rollback");

        if engine_dest_dir.exists() {
            // Backup old version
            if rollback_dir.exists() {
                std::fs::remove_dir_all(&rollback_dir).ok();
            }
            std::fs::rename(&engine_dest_dir, &rollback_dir)?;
        }

        std::fs::create_dir_all(&engine_dest_dir)?;
        println!("Extracting to {}…", engine_dest_dir.display());
        let extract_result = if platform == "windows" {
            self.extract_zip(&archive_path, &engine_dest_dir)
        } else {
            self.extract_tar_gz(&archive_path, &engine_dest_dir)
        };

        if let Err(e) = extract_result {
            println!("Extraction failed: {e}");
            std::fs::remove_dir_all(&engine_dest_dir).ok();
            if rollback_dir.exists() {
                println!("Rolling back to previous version…");
                std::fs::rename(&rollback_dir, &engine_dest_dir).ok();
            }
            bail!("Extraction failed and was rolled back.");
        }

        // 5 ── rebrand executable to helpofai naming scheme
        #[cfg(target_os = "windows")]
        let (old_name, new_name) = ("codebase-memory-mcp.exe", "helpofai-codebase-memory.exe");
        #[cfg(not(target_os = "windows"))]
        let (old_name, new_name) = ("codebase-memory-mcp", "helpofai-codebase-memory");

        let old_bin = engine_dest_dir.join(old_name);
        let new_bin = engine_dest_dir.join(new_name);
        if old_bin.exists() {
            std::fs::rename(&old_bin, &new_bin)?;
        }

        // 6 ── make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = std::fs::metadata(&new_bin).map(|m| m.permissions()) {
                perms.set_mode(0o755);
                std::fs::set_permissions(&new_bin, perms).ok();
            }
        }

        // 8 ── initialize storage directory structure
        let engine_dest_dir = engine_dir()?;
        let data_dir = engine_dest_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("codebase-memory");
        std::fs::create_dir_all(data_dir.join("repositories"))?;
        std::fs::create_dir_all(data_dir.join("global"))?;
        std::fs::create_dir_all(data_dir.join("indexes"))?;
        std::fs::create_dir_all(data_dir.join("runtime"))?;
        std::fs::create_dir_all(data_dir.join("logs"))?;

        // 9 ── confirm
        let binary = engine_binary_path()?;
        let ver = probe_version(&binary).unwrap_or_else(|_| "unknown".into());
        println!("✓ Codebase Memory engine installed: {ver}");

        // 7 ── register in mcp.json so HelpOfAi agent sessions can call it
        if let Err(e) = register_in_mcp_json(&binary) {
            // Non-fatal — user can register manually
            eprintln!("Note: MCP auto-registration skipped: {e}");
        }

        Ok(())
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    fn parse_checksum(&self, checksums: &str, target: &str) -> Result<String> {
        for line in checksums.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let fname = parts[1].trim_start_matches('*');
                if fname == target {
                    let hash = parts[0].to_lowercase();
                    if hash.len() != 64 {
                        bail!("Malformed SHA-256 in checksums.txt");
                    }
                    return Ok(hash);
                }
            }
        }
        bail!("No checksum entry for {target} in checksums.txt")
    }

    fn verify_file_hash(&self, path: &Path, expected: &str) -> Result<()> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let actual = format!("{:x}", hasher.finalize());
        if actual != expected {
            bail!("Checksum mismatch!\n  expected: {expected}\n  got:      {actual}");
        }
        Ok(())
    }

    fn extract_zip(&self, archive: &Path, dest: &Path) -> Result<()> {
        let file = File::open(archive)?;
        let mut a = zip::ZipArchive::new(file)?;
        for i in 0..a.len() {
            let mut entry = a.by_index(i)?;
            let outpath = match entry.enclosed_name() {
                Some(p) => dest.join(p),
                None => continue,
            };
            if entry.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut out = File::create(&outpath)?;
                std::io::copy(&mut entry, &mut out)?;
            }
        }
        Ok(())
    }

    fn extract_tar_gz(&self, archive: &Path, dest: &Path) -> Result<()> {
        let tar_gz = File::open(archive)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut a = tar::Archive::new(tar);
        a.unpack(dest)?;
        Ok(())
    }
}

/// Run `codebase-memory-mcp --version` and return the trimmed output.
pub fn probe_version(binary: &Path) -> Result<String> {
    let out = Command::new(binary).arg("--version").output()?;
    if !out.status.success() {
        bail!("binary exited with non-zero status");
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Write (or update) the `codebase-memory` entry in `~/.helpofai/mcp.json`.
///
/// The file is a JSON object whose top-level keys are server names.
/// We upsert a `"codebase-memory"` key pointing to the installed binary.
/// If the file doesn't exist we create it; if it does we preserve other entries.
fn register_in_mcp_json(binary: &Path) -> Result<()> {
    use serde_json::{Value, json};

    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?
        .join(".helpofai");
    std::fs::create_dir_all(&config_dir)?;

    let mcp_path = config_dir.join("mcp.json");

    let mut root: Value = if mcp_path.exists() {
        let raw = std::fs::read_to_string(&mcp_path)?;
        serde_json::from_str(&raw).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    // Upsert the codebase-memory entry (stdio MCP server)
    root["codebase-memory"] = json!({
        "command": binary.to_string_lossy(),
        "args": ["--tool-profile", "analysis", "--ui=true", "--port=9749"],
        "env": {}
    });

    let pretty = serde_json::to_string_pretty(&root)?;
    std::fs::write(&mcp_path, pretty)?;
    println!("✓ Registered codebase-memory in {}", mcp_path.display());
    Ok(())
}
