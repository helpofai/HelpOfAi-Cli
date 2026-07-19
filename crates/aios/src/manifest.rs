//! # Manifest parser
//!
//! Reads and validates a single AIOS `module.json` file.
//! Does not concern itself with registries or load ordering —
//! that is the `registry` and `loader` modules' job.

use std::path::Path;

use crate::types::ModuleManifest;

/// Read a `module.json` from disk and deserialize it.
///
/// Returns `Err` when the file is missing, not valid UTF-8,
/// or does not deserialize as a `ModuleManifest`.
pub fn parse_manifest(path: &Path) -> anyhow::Result<ModuleManifest> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;

    let manifest: ModuleManifest = serde_json::from_str(&raw).map_err(|e| {
        anyhow::anyhow!(
            "{} is not a valid AIOS module manifest: {e}",
            path.display()
        )
    })?;

    Ok(manifest)
}

/// Quick validation — checks that the manifest has the minimal required fields.
pub fn validate_manifest(m: &ModuleManifest) -> anyhow::Result<()> {
    if m.id.is_empty() {
        anyhow::bail!("module manifest has empty id");
    }
    if m.name.is_empty() {
        anyhow::bail!("module {id} has empty name", id = m.id);
    }
    if m.version.is_empty() {
        anyhow::bail!("module {id} has empty version", id = m.id);
    }
    if m.module_type.is_empty() {
        anyhow::bail!("module {id} has empty type", id = m.id);
    }
    if m.manifest_version.is_empty() {
        anyhow::bail!("module {id} has empty manifest_version", id = m.id);
    }
    Ok(())
}
