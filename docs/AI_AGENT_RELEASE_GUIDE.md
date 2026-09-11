# AI Agent Release & Version Bump Runbook

This guide defines the authoritative, step-by-step instructions for AI agents and maintainers when performing version bumps, preflight integrity verification, and release tag publishing in `helpofai/HelpOfAi-Cli`.

---

## 1. Release Architecture & Core Rules

1. **Active Branch:** Always work on `helpofai/cli-agents` (confirm with `git branch --show-current` before starting). Never commit directly to `main`.
2. **Strict Verification Gate:** Never tag or state "fixed/released" without passing all verification checks:
   - `scripts/release/check-versions.sh` must return exit code `0`.
   - `cargo fmt -- --check` must pass with 0 formatting differences.
   - `cargo clippy --workspace -- -D warnings` must compile with 0 errors and 0 warnings.
   - Targeted unit/integration tests must pass cleanly.
3. **No Partial Bumps:** The workspace, all internal `crates/*/Cargo.toml` path dependencies, `npm/helpofai/package.json`, `npm/runtime-sdk/package.json`, and `Cargo.lock` must stay exactly synchronized.

---

## 2. Step-by-Step Version Bump Procedure

When performing a release bump (e.g. `vX.Y.Z`):

### Step 2.1: Update Root Manifest
Edit `Cargo.toml`:
```toml
[workspace.package]
version = "X.Y.Z"
```

### Step 2.2: Update Internal Crate Dependency Pins
All crates under `crates/*/Cargo.toml` that depend on sibling crates using path dependencies must have their version pinned to `"X.Y.Z"`:
- `crates/agent/Cargo.toml`
- `crates/app-server/Cargo.toml`
- `crates/cli/Cargo.toml`
- `crates/config/Cargo.toml`
- `crates/core/Cargo.toml`
- `crates/execpolicy/Cargo.toml`
- `crates/hooks/Cargo.toml`
- `crates/tools/Cargo.toml`
- `crates/tui/Cargo.toml`

### Step 2.3: Update npm Packages
1. `npm/helpofai/package.json`:
   - `"version": "X.Y.Z"`
   - `"helpofaiBinaryVersion": "X.Y.Z"`
2. `npm/runtime-sdk/package.json`:
   - `"version": "X.Y.Z"`

### Step 2.4: Update Documentation & Readmes
1. `CHANGELOG.md`:
   - Add new `## [X.Y.Z] - YYYY-MM-DD` section under `### [Unreleased]`.
   - Document changes categorized under `Added`, `Changed`, `Fixed`, `Security`.
   - Update bottom compare links:
     ```markdown
     [Unreleased]: https://github.com/helpofai/HelpOfAi-Cli/compare/vX.Y.Z...HEAD
     [X.Y.Z]: https://github.com/helpofai/HelpOfAi-Cli/compare/vPREV...vX.Y.Z
     ```
2. Synchronize TUI changelog slice:
   ```bash
   bash scripts/sync-changelog.sh
   ```
3. Update README installation tags (`README.md`, `README.zh-CN.md`, `README.ja-JP.md`, `README.vi.md`):
   ```bash
   cargo install --git https://cnb.cool/helpofai.net/helpofai --tag vX.Y.Z helpofai-cli --locked --force
   cargo install --git https://cnb.cool/helpofai.net/helpofai --tag vX.Y.Z helpofai-tui --locked --force
   ```

### Step 2.5: Synchronize Lockfile
Run cargo check to lock dependencies in `Cargo.lock`:
```bash
cargo check --workspace
```

---

## 3. Automated Verification Gate Checklist

Run and verify every command before committing:

```bash
# 1. Automated version & manifest integrity check
bash scripts/release/check-versions.sh

# 2. Code formatting check
cargo fmt -- --check

# 3. Workspace Clippy check (zero warnings permitted)
cargo clippy --workspace -- -D warnings

# 4. Targeted test verification
cargo test -p helpofai-tui --bin helpofai-tui build_tool_context_uses_typed_shell_policy_per_mode
cargo test -p helpofai-aios --lib web_inspector
```

Expected output for `check-versions.sh`:
```text
Version state OK: workspace=X.Y.Z, npm=X.Y.Z, lockfile in sync.
```

---

## 4. Git Commit, Tagging, and Publishing

Once all tests and verification gates pass:

```bash
# 1. Stage all modifications
git add -A

# 2. Create standard release commit
git commit -m "release: vX.Y.Z - <summary of major additions and fixes>"

# 3. Create annotated Git tag
git tag -a vX.Y.Z -m "Release vX.Y.Z: <detailed release description>"

# 4. Push branch and tag to remote
git push origin helpofai/cli-agents
git push origin vX.Y.Z
```

GitHub Actions CI release workflow will trigger on tag push to build binaries, produce release archives, and publish npm packages.

---

## 5. Reference: Recent Key Milestones & Context

### Release v0.8.98
- **Unrestricted YOLO Mode Shell Policy**: Fixed bug in `shell_policy_for_mode` where `allow_shell = false` improperly blocked YOLO mode. YOLO mode now always returns `ShellPolicy::Full`.
- **AIOS PowerShell & Shell Execution Across All Modes**: Removed `ShellPolicy::None` restrictions from `aios_run_and_trace`, allowing AIOS diagnostic tracing and PowerShell commands on Windows across Agent, Plan, and YOLO modes.
- **AIOS Command Auto-Detection & Auto-Approval**: Added `is_aios_command` detection in `exec_shell` and `Engine::handle_run_shell_command` so `helpofai aios`, `hoa`, and AIOS workflows run without permission blocks or approval prompts.
- **Project-Local AIOS Bundle Resolution**: Extended `resolve_aios_root` to discover `.helpofai/aios` inside project directories, and updated `helpofai aios init` to target `.helpofai/aios` when project `.helpofai` exists.

### Release v0.8.97
- **Embedded AIOS Distribution**: Full AIOS bundle embedded in binary with automatic unpacking to `~/.helpofai/aios`.
- **Dual-Engine Headless Web Inspector**: Native CDP Chrome/Edge + Node/Playwright runner with real-time console/network error listener and workspace file correlation.
- **AIOS Web Debug Workflow**: Added `WORKFLOW-000011-web-debug.json` automating web inspection and error diagnostics.
