# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### [Unreleased]

## [0.8.79] - 2026-07-16

### Added

- **9 New Providers**: Added support for `DeepseekAnthropic`, `Qianfan`, `Openmodel`, `MinimaxAnthropic`, `Sakana`, `LongCat`, `Meta`, `Xai`, and `Custom` across registry, capability configurations, header mapping, and picker views.
- **Dynamic Status & Spinner Engine**: Composer box title now displays active statuses (`Generating response...`, `Compacting context...`, `Running <tool_name>...`) with smooth rotating braille spinners.
- **Optimized Render Loop Pacing**: Boosted visual frame rates by reducing active poll rates to 16ms and active animation intervals to 50ms.

### Fixed

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

## [0.8.64] - 2026-06-22

## [0.8.62] - 2026-06-17

### Changed

- **GLM-5.2 is now the default direct Z.AI model.** `DEFAULT_ZAI_MODEL` resolves
  to `GLM-5.2` in both `helpofai-tui` and `helpofai-config`; the `glm-5.1`
  alias still resolves to `GLM-5.1` (the defaulting was decoupled from the alias
  arm so it no longer tracks the default). Docs and `config.example.toml` no
  longer describe GLM-5.2 as an opt-in preview.
- **GLM-5-Turbo registered as a real model** and wired as the faster/explore
  sub-agent sibling for the GLM family: a `GLM-5.2` parent routes
  faster/explore children to `GLM-5-Turbo` (direct Z.ai) and `z-ai/glm-5-turbo`
  (OpenRouter), instead of down to GLM-5.1. GLM-5.1 and GLM-5-Turbo themselves
  have no cheaper tier and keep children on the parent.
- **`type: "explore"` sub-agents default to `model_strength: "faster"`.** Bounded
  read-only lookup/search/status work now uses the cheaper same-family sibling
  automatically, unless an explicit `model` or `model_strength: "same"` is
  supplied. Non-explore roles keep the conservative `same` default.
- **GPT-5.5 / OpenAI Codex faster route stays on GPT-5.5** with reasoning
  resolved to `low` (the Codex Responses API has no true `off`, so the resolved
  effort is now honest `low` rather than `off` silently rewritten). No
  DeepSeek/GLM fallback is fabricated when no cheaper same-provider sibling
  exists. DeepSeek Pro→Flash routing and its no-thinking faster lane are
  unchanged.
- **Base prompt / delegate skill guidance** updated to encourage parallel
  read-only exploration (2-4 `type: "explore"` sub-agents) for broad repo,
  version, branch, benchmark, and API-surface investigations, while keeping
  architecture, integration, and final verification in the parent. The
  delegate skill examples now use provider-neutral `model_strength` instead of
  hardcoded DeepSeek model ids.
- **Agent synthesis guardrails.** The base constitution now frames tools around
  sufficient evidence rather than open-ended persistence: extra reads, searches,
  and delegation must target a missing fact, and agents should answer with
  limits instead of broadening searches indefinitely. The runtime loop guard
  now blocks duplicate read-only/delegated calls earlier and caps repeated
  broad lookup/delegation loops in a single turn with a synthesis-forcing tool
  error. Guard metadata distinguishes exact duplicates
  (`identical_tool_call`) from no-progress loops (`no_progress_tool_loop`).
- **Sub-agent handoff and visibility.** Direct sub-agent completions are drained
  before the next parent model request, so finished children can wake the main
  model promptly instead of waiting for an empty-tool-use branch or idle engine
  path. Nested sub-agents now report completions to their immediate parent
  inbox; the main model still receives only direct-child completions, avoiding
  grandchild floods while preserving nested evidence flow. Sub-agent output
  guidance now requires child-agent provenance when a sub-agent relies on a
  child report: cite the child `agent_id` and the child's EVIDENCE line(s), and
  do not present child findings as directly verified facts. The sidebar orders
  sub-agents as a parent/child tree and annotates nested rows with parent and
  depth information in hover text.
- **Sub-agent summary provenance (#2652).** A sub-agent's free-text result is now
  explicitly treated as an unverified self-report rather than confirmed
  evidence. The completion sentinel carries `summary_kind: complete | truncated`
  so the parent model can branch on whether it saw the full report or a clipped
  excerpt. Short summaries (≤ 12,000 chars) get a soft "re-verify material
  claims" suffix; longer ones are head+tail truncated with an honest marker
  stating the elided middle is not retrievable via `retrieve_tool_result`.
  Every summary therefore carries exactly one boundary marker, never both.
- **Provider metadata centralization.** Provider env vars, config keys, aliases,
  and auth hints are now resolved through the shared `ProviderMetadata` registry
  across `helpofai-config`, `helpofai-tui`, and `helpofai-cli`, reducing drift
  between the provider picker, `helpofai auth`, `doctor --json`, and setup
  hints.

### Added

- **Agent clarification questions (#3102).** Agents now have a first-class
  `request_user_input` tool to ask the user structured clarifying questions
  through a modal UI surface instead of only emitting a chat message and hoping
  the user notices. Mirrors the approval/secret-request flow the harness
  already used for permissions. The tool accepts 1-3 questions, each with a
  header, an id, 2-4 selectable options (label + description), and
  `allow_free_text` / `multi_select` flags (both default to `false` for
  back-compat). Input is validated up front with actionable errors. Wired
  across all layers: the `request_user_input` tool, engine handling
  (`turn_loop` → `approval`), an interactive TUI modal (`UserInputView`) with
  full keyboard navigation, and the runtime protocol
  (`EventFrame::UserInputRequest` + `AppRequest::SubmitUserInput`) so headless
  / app-server clients can answer programmatically. Parity tests cover the
  wire round-trip and the omitted-flags default.
- **Transcript hyperlinks — out-of-band OSC 8 (#3029).** Clickable file /
  file:line / URL links now reach the terminal through a column-drift-safe
  path. Link payloads are embedded in-band by the markdown renderer, then
  extracted out of the ratatui buffer cells and re-emitted out-of-band by
  `ColorCompatBackend` — so the `ESC` bytes never occupy display columns or
  corrupt selection. Supporting terminals get live hyperlinks; others see the
  label text unchanged. Clipboard/selection extraction strips residual codes as
  defense-in-depth.
- **HelpOfAi-only skill discovery gate (#3296).** New
  `[skills].scan_helpofai_only = true` limits session-time skill discovery to
  HelpOfAi-owned roots (`<workspace>/.helpofai/skills`, `~/.helpofai/skills`,
  and any explicit `skills_dir`) while ignoring cross-tool directories such as
  `.claude/skills`, `.opencode/skills`, `.cursor/skills`, and `~/.agents/skills`.
  The default remains the broad compatibility scan.
- **Permission/ask runtime rules (#3295).** Sibling `permissions.toml` ask-only
  rules are now loaded by the TUI engine and applied to `exec_shell` before
  Auto/session approval shortcuts. Matching ask rules force an approval prompt
  in otherwise auto-approved flows and are rejected under
  `approval_mode = "never"`.
- **Runtime API no-auth documentation.** `docs/RUNTIME_API.md` now documents
  `helpofai app-server --insecure-no-auth` for loopback-only testing and warns
  against combining it with `--mobile` on `0.0.0.0`.

### Fixed

- **TUI polish.** The empty-startup welcome block is centered by the actual
  rendered text width, fixing the off-center layout left over from the old
  sidebar-oriented welcome composition. Streaming HTTP body read errors now
  explain whether HelpOfAi can retry before output, or is surfacing a warning
  after partial output to avoid replaying and duplicating streamed text.
- **Config comment preservation.** Rewriting `config.toml`, `settings.toml`, or
  `tui.toml` now merges user comments and formatting back into the serialized
  document; if comment merge fails, the write falls back to plain serialized
  output rather than failing.
- **Snapshot gate respected for per-tool snapshots (#3292).** Per-tool snapshots
  now check `[snapshots].enabled` before writing, matching the existing
  session-level gate.
- **Poppler `pdftotext` detection (#1667).** The dependency resolver now probes
  `pdftotext -v` instead of `--version`, because Poppler treats `--version` as
  an input filename. Fixes detection on systems where only Poppler is installed.
- **Plan confirmation checklist visibility.** The Plan-mode confirmation modal
  now shows the active checklist under the plan details, so users can review the
  concrete `checklist_write` work breakdown before accepting or revising a plan.

### Retroactive credits

A credit-reconciliation pass found shipped community fixes that were never
recorded in this changelog. Crediting them now, with the version they shipped in:

- Global `~/.deepseek/AGENTS.md` fallback loading — thanks @manaskarra (fix) and @xfy6238 (report) (#1157, v0.8.27)
- CRLF SSE event parsing for MCP — thanks @reidliu41 (fix) and @djairjr (report) (#1309, v0.8.29)
- Reduce-motion default on VTE/flicker terminals — thanks @Geallier (report) (#1470, v0.8.34)
- `portable-pty` 0.9 upgrade for LoongArch64 — thanks @quentin-lian (fix) and @k0tran (report) (#1531, #1992, v0.8.46)
- `DEEPSEEK_ALLOW_INSECURE_HTTP` guard for LAN vLLM — thanks @F1LT3R (report) (#1656, v0.8.47)
- Hidden `reasoning_content` kept in English regardless of locale — thanks @cmyyy (report) (#1842, v0.8.47)
- `ExternalTool` abstraction layer — thanks @aboimpinto (#1794, #2294, v0.8.48)
- Ephemeral generated project context — thanks @Final527 (report) (#3058, v0.8.59)

---

Older releases: [CHANGELOG.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/CHANGELOG.md) and [docs/CHANGELOG_ARCHIVE.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/CHANGELOG_ARCHIVE.md).
