use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let upstream_dir = manifest_dir.join("../../ext/codebase-memory-upstream");
    
    if !upstream_dir.exists() {
        println!("cargo:warning=Upstream submodule not found at {:?}. Have you initialized submodules?", upstream_dir);
        return;
    }

    println!("cargo:rerun-if-changed={}", upstream_dir.display());

    // Instead of rebuilding the entire complex C11+TreeSitter build system in cc::Build,
    // we invoke the upstream Makefile.cbm directly wrapper, then rename the binary.
    let status = Command::new("make")
        .arg("-f")
        .arg("Makefile.cbm")
        .arg("cbm-with-ui")
        // Pass rebranding definitions via CFLAGS overlay
        .env("CFLAGS_COMMON", "-DPRODUCT_NAME=\"helpofai-codebase-memory\" -DCBM_VERSION=\"0.10.8-helpofai\" -include src/helpofai_rebrand.h")
        .current_dir(&upstream_dir)
        .status();

    match status {
        Ok(s) if s.success() => {
            // Move and rename the compiled binary to the OUT_DIR
            let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
            let src_bin = upstream_dir.join("codebase-memory-mcp");
            let src_bin_windows = upstream_dir.join("codebase-memory-mcp.exe");
            let dest_bin = out_dir.join(if cfg!(target_os = "windows") { "helpofai-codebase-memory.exe" } else { "helpofai-codebase-memory" });
            
            if src_bin.exists() {
                std::fs::rename(src_bin, dest_bin).expect("Failed to move compiled binary");
            } else if src_bin_windows.exists() {
                std::fs::rename(src_bin_windows, dest_bin).expect("Failed to move compiled binary");
            }
        },
        Ok(_) => {
            println!("cargo:warning=make failed, but we will fallback to the binary downloader for now (or fail silently on Windows if make is missing)");
        },
        Err(e) => {
            println!("cargo:warning=Failed to invoke make: {}. Falling back to prebuilt binary strategy.", e);
        }
    }
}
