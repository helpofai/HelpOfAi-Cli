# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### [Unreleased]

## [0.8.93] - 2026-07-22

### Added

- **Multi-Terminal View**: Added a new multi-terminal TUI pane (accessible via `Ctrl+t`) to track live stdout/stderr streams from active background tasks.
- **Smart Build Log Filter**: Implemented an AIOS smart error filter (`aios_run_and_trace`) to extract, deduplicate, and compact compilation errors for AI ingestion, reducing log token bloat up to 90%.
- **Live Background Task UI Hooks**: Integrated real-time AIOS process blinking and shell job ping-back notifications directly into the main TUI render loop.

## [0.8.92] - 2026-07-21

### Added

- **Global AIOS Root Resolution**: Implemented `resolve_aios_root` to auto-discover AIOS bundles across workspace directories, `$HELPOFAI_AIOS_DIR`, `~/.helpofai/aios`, and executable sibling paths.
- **Branded `!hoa` & `!aios` TUI Autocomplete**: Added interactive `!` bang menu supporting `!hoa`, `!aios`, and direct workflow shortcuts (`!hoa build-feature`, `!hoa fix-bug`, `!hoa review`, `!hoa refactor`, `!hoa audit`, `!hoa health`, `!hoa brain`).
- **Live `AIOS Active` Status Badge**: Added a green `AIOS Active` status chip in the TUI footer bar whenever AIOS integration is enabled.
- **Workspace Auto-Session Resume & Picker**: Implemented automatic workspace session detection on TUI launch, auto-resuming single sessions and prompting with an interactive session list when multiple project sessions exist.

## [0.8.91] - 2026-07-20

### Added

- **AIOS Specialist Agent Routing**: Connected TUI `SubAgentManager` to dynamically load the AIOS Agent Registry and inject specialist system prompts (e.g. Master, Architect, Backend, QA) when spawning subagents.
- **Stage-by-Stage Phase Execution**: Added a stateful workflow run loop to the CLI `aios run` command that executes each lifecycle phase (e.g. `understand`, `implement`, `validate`) via headless TUI subagents.
- **Capability-based Sandboxing**: Implemented dynamic capability filtering inside `SubAgentToolRegistry` to restrict subagent tool visibility and access to only those authorized by the agent's declared capability contract.
- **Decision Journal & Timeline Log**: Added stateful logging of workflow runs, writing detailed JSON execution journals to `aios/runs/run_<workflow>_<timestamp>.json` containing phase durations, outcomes, and gate approvals.
- **AIOS Project Brain & Code Knowledge Graph**: Integrated SQLite-backed AST code parser, symbol indexer, and multi-language Code Knowledge Graph (`helpofai aios brain-index`) to index workspace symbols with zero AI token cost.
- **Multi-File Impact & Ripple Analysis Engine**: Implemented workspace-wide caller dependency tracking (`helpofai aios brain-impact`) to pre-compute affected files and prevent silent breaking changes during refactorings.

## [0.8.89] - 2026-07-17

### Fixed

- **Retry for clean empty streams**: Ensure the CLI automatically retries up to 3 times if a gateway stream completes normally but yields no text or thinking content (e.g. from local unauthenticated OmniRoute providers).
- **Clearer empty response warnings**: Updated the default empty response status message to guide users to check their provider/gateway configuration rather than outputting a misleading reasoning-only warning.

## [0.8.88] - 2026-07-17

### Added

- **9 New Providers**: Added support for `DeepseekAnthropic`, `Qianfan`, `Openmodel`, `MinimaxAnthropic`, `Sakana`, `LongCat`, `Meta`, `Xai`, and `Custom` across registry, capability configurations, header mapping, and picker views.
- **Dynamic Status & Spinner Engine**: Composer box title now displays active statuses (`Generating response...`, `Compacting context...`, `Running <tool_name>...`) with smooth rotating braille spinners.
- **Optimized Render Loop Pacing**: Boosted visual frame rates by reducing active poll rates to 16ms and active animation intervals to 50ms.

### Fixed

- **OmniRoute default auto model**: Prevent root-level `default_text_model` overrides from forcing DeepSeek models on OmniRoute, ensuring it uses `"auto"` by default.
- **OmniRoute ModelRegistry resolution**: Added default `"auto"` model registry entry and passthrough resolving for OmniRoute, preventing fallback to DeepSeek when querying the registry or resolving models.
- **Transparent Reasoning-Only Retry**: Automatically retry when a reasoning model returns thinking but fails to yield a final answer or tool calls.
- **Hermetic Unit Testing**: Hardened environment isolation in DeepSeek defaults test to prevent host configuration leakage.

## [0.8.78] - 2026-07-15

### Added

- Support for dynamically fetching and selecting models from the OmniRoute gateway in the Model Picker.

### Fixed

- Friendly network error connection message when the local OmniRoute server is offline.

## [0.8.77] - 2026-07-15

### Added

- OmniRoute provider integration is now fully functional and passes all tests.

### Fixed

- Adjusted DeepSeek base URL logic to avoid env overrides when base_url is not set (fixes test).

## [0.8.76] - 2026-07-15

### Fixed

- **OmniRoute auto-router bypass**: Completely bypass the local classification router when OmniRoute is the active provider. This prevents out-of-band requests to `deepseek-v4-flash` and avoids DeepSeek credential requirements altogether for OmniRoute setups.

## [0.8.75] - 2026-07-15

### Fixed

- **OmniRoute auto-router authentication**: Dynamically resolve the auto-routing classifier provider to OmniRoute (or DeepSeek China) when it is active, avoiding missing DeepSeek credential errors.

## [0.8.74] - 2026-07-14

### Fixed

- **OmniRoute stream reasoning**: Treat OmniRoute as always-reasoning in the SSE stream parser to correctly capture and render thinking/reasoning blocks when using the `auto` routing model.

## [0.8.73] - 2026-07-14

### Fixed

- Provider logic issues and aligned with CodeWhale structure.

## [0.8.72] - 2026-07-14

### Fixed

- **qa_pty cursor-read reliability**: The PTY test harness now answers
  every cursor-position (DSR `ESC[6n`) query from the headless terminal,
  including queries split across PTY reads or duplicated within a single
  read. A missed query previously left crossterm blocking until it
  surfaced "The cursor position could not be read within a normal
  duration" onto the screen, which corrupted the viewport and broke
  `viewport_origin_stays_row_zero_after_failed_turn`. The query is also
  now stripped from the parsed terminal stream.

## [0.8.71] - 2026-07-13

### Fixed

- **OmniRoute localhost auth**: The CLI was sending an empty API key to
  `http://localhost:20128` because the localhost shortcut in
  `deepseek_api_key()` applied to every provider. OmniRoute is a gateway
  that requires its own token even on localhost; the shortcut now skips
  `ApiProvider::Omniroute` so the configured gateway key reaches the
  server.
- **qa_pty failed-turn flakiness**: The viewport-origin test used
  `http://invalid.test`, whose DNS timeout exceeded the wait window on
  Windows. Replaced with `127.0.0.1:1`, an unlistening local port that
  refuses instantly with no DNS wait.

## [0.8.70] - 2026-07-13

### Added

- **OmniRoute provider**: Registered the OmniRoute free AI gateway
  (`http://localhost:20128/v1`) as a first-class provider so
  HelpOfAi/cli can route through its 237-provider catalog from one
  endpoint. Defaults to the local gateway and the smart `auto` router.
- **Gateway model passthrough**: OmniRoute is a model-passthrough
  provider, so routing instructions reach the gateway verbatim —
  `auto`, `auto/coding`, `auto/cheap`, `cc/claude-opus-4-7`,
  `glm/glm-5.1`, etc. The `/model` picker is populated from
  OmniRoute's own `/v1/models` catalog via the existing discovery path.
- **Reasoning + chat-completions dialect**: OmniRoute speaks
  OpenAI chat completions and transcodes upstream, so HelpOfAi
  always uses the chat-completions dialect while the gateway resolves
  the real upstream model and its thinking/reasoning support.

### Fixed

- **OmniRoute build + test stabilization**: Completed the `ApiProvider::Omniroute`
  wiring (default model/base-url constants, `ProvidersConfig` merge, and
  reasoning-effort match arms) so the provider compiles, and stabilized four TUI
  tests against environment-specific state (Windows `\\?\` path prefix, CRLF
  prompt line endings, and `auto_model` isolation from local settings).

## [0.8.69] - 2026-07-08

### Changed

- **Clean Up**: Completely removed legacy `deepseek-tui` npm package and environment variable shims in favor of the canonical `helpofai` equivalents.

## [0.8.68] - 2026-07-07

### Fixed

- **Memory Auto-Init**: Auto-create `~/.helpofai/memory.md` and `.helpofai/memory.md` with usage instructions when memory is enabled for the first time.
- **Update Notification**: Show current version, latest version, and upgrade commands at startup when a newer release is available.
- **Borrow Checker**: Fixed move-after-use compile error in `App::new` by reading `memory_path` and `workspace` from the constructed `app` instance.

## [0.8.67] - 2026-07-07

### Added

- **Memory Settings Modal**: Added interactive sub-TUI popup to enable/disable user memory, configure max size, and set custom file paths via `/memory config`.

## [0.8.66] - 2026-06-23

### Fixed

- **Context Freeze**: Persist generated context instructions to `.helpofai/instructions.md` to prevent repeated filesystem scans on startup.
- **Starlark Compatibility**: Pinned `starlark` to `0.13.0` across workspace (including `helpflow` and `tui`) to fix trait bound compilation errors (`allocative`) on Rust 1.88.

---

Older releases: [CHANGELOG.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/CHANGELOG.md) and [docs/CHANGELOG_ARCHIVE.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/CHANGELOG_ARCHIVE.md).
