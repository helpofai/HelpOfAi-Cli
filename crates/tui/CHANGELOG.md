# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### [Unreleased]

## [0.9.1] - 2026-09-16

### Added
- **Codebase Memory Engine Integration**: Added native support for the DeusData Codebase Memory engine. Includes `helpofai codebase install`, `helpofai codebase update`, `helpofai codebase doctor`, and `helpofai graph` UI supervisor, isolating the CBM graph dynamically in `%LOCALAPPDATA%` and auto-registering the codebase-memory MCP engine for AI agent consumption.
- **Built-in 3D Graph Visualization**: Bundled 3D interactive knowledge graph visualization assets with real-time process monitoring and graph rendering.
- **Dynamic Port Management & Process Supervisor**: Added automatic port reservation, stale PID cleanup, and lifecycle management for the codebase memory server.

## [0.8.99] - 2026-09-11

### Added
- **AIOS Full Platform Exposure to AI Agents**: Integrated all native AIOS subsystems, engines, and architectural catalogs as first-class tools for AI agents in Agent Mode and Plan Mode.
- **Headless Web Inspector & Console Error Tool (`aios_web_inspect`)**: Enables agents to inspect web applications (`http://localhost:3000`, `127.0.0.1`, LAN IPs, remote URLs), capturing rendered page content, browser console errors (`console.error`, `console.warn`), JavaScript runtime exceptions, and failed HTTP requests (4xx/5xx/CORS), with automatic correlation to workspace source files and suggested code fixes.
- **AIOS Workflow Execution History**: Added `action: 'history'` to `aios_workflow` allowing AI agents to inspect past workflow runs, execution statuses, duration timelines, and phase logs from `aios/runs/`.
- **AIOS Constitution, Architecture Patterns & Templates Inspection**: Added `constitution`, `patterns`, and `templates` actions to `aios_registry`, allowing AI agents to retrieve the 15 immutable engineering laws, architectural patterns, and standard engineering templates (`PLAN.md`, `ADR_TEMPLATE.md`, `BUG_REPORT.md`, `SECURITY_REPORT.md`, `ROLLBACK_PLAN.md`, etc.).
- **Enterprise Workspace Operations & Modernization Analyzer**: Added `aios_workspace` (`collect`, `read`, `write`, `copy`, `move`, `delete`, `rollback`, `analyze_upgrade`) with atomic writes, pre-image hashing, snapshot rollback journals, multi-file context collection, and tech-stack modernization analysis.
- **Optimized Project Brain Indexing**: Codebase scanner with directory pruning (`vendor`, `node_modules`, `storage`, `target`, `dist`, `build`, etc.) and BLAKE3 incremental change caching for instant knowledge graph indexing without context overflow.

## [0.8.98] - 2026-09-11

### Fixed
- **YOLO Mode Shell Policy Enforcement**: Fixed an issue where `shell_policy_for_mode` returned `ShellPolicy::None` in YOLO mode if `allow_shell` was not explicitly configured in `config.toml`, which caused "Shell tools are disabled by the active permission profile" errors. YOLO mode now always grants `ShellPolicy::Full`.
- **Unrestricted AIOS PowerShell & Shell Execution Across All Modes**: Removed `ShellPolicy::None` permission checks from `aios_run_and_trace`, allowing AIOS diagnostic tracing and PowerShell execution across Agent, Plan, and YOLO modes.
- **AIOS Command Auto-Detection & Auto-Approval**: Added `is_aios_command` detection in `exec_shell` and `Engine::handle_run_shell_command` so `helpofai aios`, `hoa`, and AIOS workflows are permitted and auto-approved without manual confirmation prompts.
- **Project-Local AIOS Bundle Resolution (`.helpofai/aios`)**: Extended `resolve_aios_root` to discover `.helpofai/aios` inside project folders, and updated `helpofai aios init` to initialize into `.helpofai/aios` when a project `.helpofai` directory is present.

## [0.8.97] - 2026-09-11

### Added
- **Embedded AIOS Distribution & Auto-Unpack**: The full AIOS bundle (`aios.json`, schemas, constitution, 28 modules, 34 capabilities, workflows, agents) is now embedded directly into the binary with compile-time zero-download guarantee. The binary automatically unpacks to `~/.helpofai/aios` on first run if not present, and official release packages and install scripts now ship the bundle out of the box. Added `helpofai aios init` command.
- **Dual-Engine Headless Web Inspector (`web_inspect` & `helpofai web-inspect`)**: Native headless browser inspection via Chrome/Edge DevTools Protocol (CDP) using pre-installed system browsers with zero extra dependencies, along with Node.js/Playwright/Puppeteer runner support and HTTP fallback.
- **Real-Time Console & Network Error Listener**: Automatically captures `console.error`, `console.warn`, and uncaught JavaScript runtime exceptions with file names, line numbers, column numbers, and stack traces, plus failed HTTP 4xx/5xx, CORS, and connection-refused network requests.
- **Automated Workspace Correlation & Auto-Fix Loop**: Automatically correlates browser runtime errors to matching local workspace source files (`.jsx`, `.tsx`, `.vue`, `.svelte`, `.js`, `.html`), extracts code line context snippets, and generates targeted fix recommendations for the AI agent to patch code.
- **AIOS Web Debug Workflow**: Added `WORKFLOW-000011-web-debug.json` (`hoa debug-web`) automating the inspect -> correlate -> auto-fix -> re-verify loop.

## [0.8.96] - 2026-09-11

### Added

- **AIOS First-Class Tools Suite (`aios_workflow`, `aios_brain`, `aios_registry`, `aios_run_and_trace`)**: Upgraded model-facing bridge tools providing full control over AIOS execution—including listing, inspecting, diagnosing, and running multi-phase workflows with journal persistence, querying the Project Brain knowledge graph, executing multi-file blast-radius impact analysis, inspecting 28 modules and 34 capabilities, and extracting specialist agent system prompts.
- **Enterprise Constitution & Golden Rules Injection**: Automatically compiles and injects `aios/constitution.json` into model prompts when `aios/` is present, binding models to the Golden Rule, 15 engineering principles, and supremacy ordering.
- **Subcrate AIOS Root Discovery**: Enhanced bundle resolution to walk parent directory ancestors, ensuring AIOS tools and workflows resolve reliably across subdirectories and subcrates.

## [0.8.95] - 2026-09-02

### Added

- **Local Network & Device Inspection (`allow_local_network`)**: Added configurable local network support across `fetch_url` and network policy, allowing inspection of `localhost`, `127.0.0.1`, and private LAN subnets (`192.168.x.x`, `10.x.x.x`) with strict protection against cloud metadata SSRF targets (`169.254.169.254`).
- **Separate OS Window Terminal Engine**: Added native cross-platform detached terminal window launching (`new_window: true` in `exec_shell` and `w` shortcut in `Ctrl+T` Multi-Terminal) for Windows Terminal, PowerShell, macOS Terminal.app, and Linux terminal emulators.
- **AIOS Enterprise Sub-Agent Fleet**: Upgraded `SubAgentType` and `AgentTool` to natively recognize and validate all 14 AIOS specialized enterprise roles (`architect`, `backend`, `frontend`, `database`, `api`, `qa`, `devops`, `security`, `documentation`, `android`, `ios`, `flutter`, `laravel`, `react`), dynamically loading their companion prompt instructions.

## [0.8.94] - 2026-08-04

### Added

- **Arbitrary Background Commands (`&`)**: Support running any shell command asynchronously in the background by appending `&` (e.g. `!cargo build &`). Status is tracked dynamically via `/jobs`.
- **TTY State Recovery Commands**: Added `/reset` and `/reset-tty` TUI commands to restore raw terminal mode, mouse capture, and viewport states if a child process crashes or leaves the console unstable.
- **Smart AIOS Task Autocomplete & Fallbacks**: Integrated generic task routing (`!aios <task>`) to auto-dispatch workflow runs, and added smart default parameter strings so common actions like `!aios review` run without requiring explicit arguments.

### Fixed

- **Resilient Cursor Probes**: Handled `crossterm` cursor position timeout errors gracefully, returning a default `(0, 0)` fallback instead of panicking the TUI when raw-mode reads are delayed (specifically resolving startup and runtime crashes on Kali Linux).

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

---

Older releases: [CHANGELOG.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/CHANGELOG.md) and [docs/CHANGELOG_ARCHIVE.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/CHANGELOG_ARCHIVE.md).
