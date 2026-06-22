# Rebrand: DeepSeek TUI → HelpOfAi

Starting with **v0.8.41**, this project ships under a new name: `helpofai`.

This document explains what changed, what didn't, and how to migrate. None of the
DeepSeek provider integration changed — only the local CLI / TUI brand.

## TL;DR

```bash
# 1. Uninstall the old wrapper or binaries.
npm uninstall -g deepseek-tui      # or:
cargo uninstall deepseek-tui-cli 2>/dev/null || true
cargo uninstall deepseek-tui 2>/dev/null || true
                                    # legacy Homebrew installs may use:
                                    # brew upgrade deepseek-tui

# 2. Install under the new name.
npm install -g helpofai            # or:
cargo install helpofai-cli --locked
cargo install helpofai-tui --locked
                                    # legacy Homebrew installs may still use
                                    # brew install deepseek-tui until the tap
                                    # formula is renamed.

# 3. Run with the new command.
helpofai doctor
helpofai
```

Your existing `~/.deepseek/config.toml`, `~/.deepseek/sessions/`,
`~/.deepseek/skills/`, `~/.deepseek/tasks/`, and `~/.deepseek/mcp.json` are
not deleted. New HelpOfAi installs prefer `~/.helpofai/`, and legacy
`~/.deepseek/` state remains a read fallback while you migrate. Existing
`DEEPSEEK_*` environment variables continue to work.

## What got renamed

| Surface | Before | After |
|---|---|---|
| CLI dispatcher binary | `deepseek` | `helpofai` |
| TUI runtime binary | `deepseek-tui` | `helpofai-tui` |
| npm wrapper package | `deepseek-tui` | `helpofai` |
| Crates.io crates | `deepseek-tui-cli` / `deepseek-tui` / `deepseek-*` | `helpofai-cli` / `helpofai-tui` / `helpofai-*` |
| Release assets | `deepseek-<platform>` / `deepseek-tui-<platform>` | `helpofai-<platform>` / `helpofai-tui-<platform>` |
| Checksum manifest | `deepseek-artifacts-sha256.txt` | `helpofai-artifacts-sha256.txt` |

## What changed for local state

New installs write product-owned state under `~/.helpofai/`. Existing
`~/.deepseek/` config, sessions, skills, tasks, MCP config, memory, and notes
remain readable as legacy fallbacks while you migrate. HelpOfAi never deletes
the legacy directory automatically.

## What did NOT change

Anything that targets the DeepSeek provider API stays exactly as it was:

- **Environment variables**: `DEEPSEEK_API_KEY`, `DEEPSEEK_BASE_URL`,
  `DEEPSEEK_MODEL`, `DEEPSEEK_PROVIDER`, `DEEPSEEK_PROFILE`, `DEEPSEEK_YOLO`,
  `DEEPSEEK_LOG_LEVEL`, plus the existing `DEEPSEEK_TUI_*` runtime knobs
  (`DEEPSEEK_TUI_BIN`, `DEEPSEEK_TUI_RELEASE_BASE_URL`, etc.). They're kept
  for backward compatibility; renaming them would break every shell rc on
  the planet.
- **Model IDs**: `deepseek-v4-pro`, `deepseek-v4-flash`, and the legacy
  aliases `deepseek-chat` and `deepseek-reasoner`.
- **Hosts**: `api.deepseek.com` (global) and `api.deepseeki.com` (China
  fallback).
- **GitHub repository URL**: `https://github.com/helpofai/HelpOfAi-Cli`.
  The old `helpofai/DeepSeek-TUI` URL redirects there during the transition.
- **Homebrew tap and formula** (`helpofai/homebrew-deepseek-tui`): still uses
  the legacy formula name for existing installs. Treat it as compatibility-only
  until the tap is renamed; new install docs prefer `helpofai` npm, Cargo,
  Docker, or direct downloads.
- **Docker image**: `ghcr.io/helpofai/helpofai`.

## Deprecation shims (removed in v0.9.0)

To keep existing shell aliases, scripts, and CI working through the rename,
v0.8.41 and later v0.8.x releases shipped **deprecation shims**:

- A `deepseek` binary that prints a one-line warning to stderr and forwards
  argv to `helpofai`.
- A `deepseek-tui` binary that does the same for `helpofai-tui`.
- The legacy `deepseek-tui` npm package is deprecated and no longer receives
  new releases. Install the `helpofai` npm package instead.

These binary shims are removed in **v0.9.0**. DeepSeek provider support, model
IDs, `DEEPSEEK_*` environment variables, and legacy `~/.deepseek/` state
fallbacks remain supported.

## Migrating in practice

### npm

```bash
npm uninstall -g deepseek-tui
npm install -g helpofai
```

### Cargo

```bash
cargo uninstall deepseek-tui-cli 2>/dev/null || true
cargo uninstall deepseek-tui 2>/dev/null || true
cargo install helpofai-cli --locked
cargo install helpofai-tui --locked
```

Or in a checkout:

```bash
cargo install --path crates/cli --locked --force
cargo install --path crates/tui --locked --force
```

### Legacy `deepseek update`

Current v0.8.x compatibility binaries recognize when they are running under a
legacy `deepseek` or `deepseek-tui` filename. In that case, `deepseek update`
or `deepseek-tui update` downloads the canonical HelpOfAi release assets and
installs them beside the legacy binary as `helpofai` and `helpofai-tui` when
the install directory is writable.

If that update path cannot write to the install directory, use the npm, Cargo,
Homebrew, or manual reinstall commands above. The legacy npm package
`deepseek-tui` remains deprecated and is not republished; npm users should move
to `npm install -g helpofai`.

### Homebrew

The tap formula still installs through the legacy `deepseek-tui` name for
existing Homebrew users. Keep using `brew upgrade deepseek-tui` only for that
compatibility path. New installs should prefer npm, Cargo, Docker, or direct
downloads until the formula and tap repo are renamed.

### Manual / GitHub Releases

`v0.8.41` through `v0.8.x` Releases attached both the canonical
`helpofai-*` / `helpofai-tui-*` assets and compatibility-only
`deepseek-*` / `deepseek-tui-*` shim assets. Starting in v0.9.0, Releases attach
only the canonical `helpofai-*` / `helpofai-tui-*` assets and the canonical
`helpofai-artifacts-sha256.txt` checksum manifest. Install or update through
`helpofai` before moving to v0.9.0.

### Sessions, skills, and manual workspaces

Renaming the binary does not require starting over:

- **Config**: on first launch, HelpOfAi copies `~/.deepseek/config.toml` to
  `~/.helpofai/config.toml` if the HelpOfAi file does not already exist.
  It never overwrites a newer HelpOfAi config. You can inspect the active path
  with `helpofai doctor`.
- **Sessions and tasks**: managed state is read from `~/.helpofai/...` when
  present, with `~/.deepseek/...` used as the legacy fallback when only the old
  directory exists. Existing saved sessions still appear in `helpofai sessions`
  and the TUI resume picker.
- **Skills**: HelpOfAi discovers workspace skills first, then global skills,
  including both `~/.helpofai/skills` and legacy `~/.deepseek/skills`. Existing
  skill directories with `SKILL.md` do not need to be rewritten.
- **MCP config**: the default path is `~/.helpofai/mcp.json`. If that file is
  absent, HelpOfAi still reads legacy `~/.deepseek/mcp.json`. To use a custom
  MCP config file, set `mcp_config_path` in `config.toml` or
  `DEEPSEEK_MCP_CONFIG`.
- **Manual binary installs**: keep the dispatcher and TUI binaries as siblings
  on your `PATH`: `helpofai` plus `helpofai-tui`. On Windows, the recommended
  user-local location is `%LOCALAPPDATA%\Programs\HelpOfAi\bin`. On Unix-like
  systems, any user-writable `PATH` directory is fine as long as both binaries
  are present.
- **Specified work directories**: running `helpofai` from a project directory,
  or launching it with a specific workspace path, does not move project files.
  HelpOfAi reads `<workspace>/.helpofai/config.toml` first and falls back to
  legacy `<workspace>/.deepseek/config.toml` when the new path is absent.

If both `~/.helpofai/...` and `~/.deepseek/...` copies exist, the HelpOfAi
path wins. Keep the legacy directory until you have confirmed `helpofai
doctor`, `helpofai sessions`, and your expected skills all show the same state.

## Why the name change

HelpOfAi is a shorter, terminal-friendlier handle for the same terminal
coding agent and the longer-term product direction: a DeepSeek-first agentic
terminal for open source and open-weight coding models. The project name,
command names, package names, release assets, Docker image, and CNB mirror move
to HelpOfAi; the official DeepSeek provider, model IDs, env vars, and
`~/.deepseek/` config surface remain first-class.

## Reporting issues with the rename

If your install broke during the migration, please open an issue at
<https://github.com/helpofai/HelpOfAi-Cli/issues> and include:

- The output of `helpofai --version` (or `deepseek --version` if you're
  still on the shim).
- Which install path you used (npm, cargo, brew, manual).
- The exact command you ran and the full error output.

We'll prioritize migration regressions.
