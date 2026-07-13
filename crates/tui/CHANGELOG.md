# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

## [0.8.61] - 2026-06-15

This release lands the **runtime control plane** for multi-agent work: the TUI stays
responsive while sub-agents run, sub-agents converge toward fleet-style durable workers
with per-role model routing, and provider/model routes are isolated per session. It also
folds in several community contributions.

### Added

- **HelpFlow runtime foundations** — worker runtime profiles (role / permissions / shell /
  tools / model-route, with non-escalating child derivation), a cross-provider model registry
  with offline catalog hydration, and provider-readiness / context-budget / provider-adapter /
  resource-telemetry services. (#3217, #3071, #3072, #3073)
- **Per-role, heterogeneous-model sub-agent routing** — sub-agents can be assigned a model and
  provider per role (e.g. scout vs. synthesis; verifiers route to a fast model). (#2027, #1768)
- **Durable goal mode** — cross-turn goal progress with token/time accounting and a
  verifier-as-judge gate before a goal may complete. (#3215, #891, #1976, #2058, #2029)
- Parent-visible worker interaction contract — a recommended action per worker. (#3226)
- Maintainer GitHub workflow skills; ACP registry submission prepared. (#3192)
- OpenAI-compatible `/v1/chat/completions` endpoint on the legacy app-server HTTP transport,
  provider-neutral, with model registry resolution and configured-credential forwarding.

### Changed

- **Sub-agents converge toward fleet-style durable workers** — real worker lifecycle states are
  projected to the sidebar instead of a hardcoded "running", and a sub-agent returns a structured
  needs-input checkpoint instead of parking. (#3226, #3096, #3154)
- The per-turn runtime tag exposes capability posture instead of human-facing mode labels. (#3213)
- Independent shell and verifier work defaults to background jobs with nonblocking waits and a
  completion notification; blocking now requires an explicit wait. (#3212)
- Sub-agent launches now expose explicit `model_strength` and `thinking` controls to the model
  instead of hidden child-model auto-routing; `explore` work is documented as a good fit for
  faster models and `thinking: "off"`.
- Plan mode is strictly read-only (no shell tools), consistent with its runtime posture.
- `/swarm` is gated behind the durable worker substrate. (#3218)
- Legacy `deepseek` install/update path resolves to `helpofai`. (#2960, #2924, #2917)

### Fixed

- **TUI freeze when multiple sub-agents spawn (launch blocker)** — the terminal input pump runs
  off the render thread, AgentProgress events are coalesced, and sub-agents no longer park on
  input with no orchestrator to answer; a six-worker stress test guards input/render/cancel
  liveness. (#3216, #3096)
- Idle sub-agent completion notifications now resume the parent turn instead of waiting for a
  later user message; thanks @giovanni-paolilla for the deadlock report (#3266).
- **Provider/model route isolation** — provider and model state is session-local, and a
  mismatched provider+model tuple is rejected at the route boundary. (#3227)
- Route-effective context-window metadata, over-limit preflight, and bounded recovery from
  `context_length_exceeded` instead of re-looping. (#3204)
- Synchronous tools (`file_search`, `grep_files`, `list_dir`) are cancellable and no longer hold
  a turn open against cancellation. (#1791)
- MCP stdio proxy startup prompts no longer strand YOLO / non-interactive runs. (#2475)
- Stalled / failed background-shell recovery; configurable sub-agent API timeout. (#1737, #1786, #1806)
- Composer: reliable queued steering + Ctrl+S send (#3203, #3224); footer busy/idle indicator
  (#2982); CJK word-wrap (#963); clickable sidebar stop targets (#3028); live token throughput
  (#3190); auto-expiring terminal sub-agent cards (#3078).
- Linux glibc preflight in the installer/update path with a clear error. (#3207, #1067)
- Self-update retries transient GitHub metadata/asset failures and falls back from the GitHub
  REST API to the public `releases/latest` redirect before constructing release asset URLs. (#3232)
- Provider picker lists providers in neutral alphabetical order instead of hard-coding DeepSeek first; the active provider stays pre-selected. (#3076)
- Work sidebar no longer shows stale `phase now:` / `phase next:` strategy rows once the checklist
  is 100% complete.
- Plan mode no longer shortcuts investigation for requests that name a repository, URL, version,
  release, build state, benchmark, bug, PR, issue, API surface, or local code path.
- Oversized pasted text stays editable in the composer, with a file backup appended at submit
  time for model access; thanks @idling11 (#3267, closes #3263).
- Bare digit keys `1`-`8` now insert text instead of firing hotbar slots; use `Alt+digit` for
  hotbar actions. Thanks @wjq2026 for the report and @DieMoe233 for the paste-path note (#3243).
- Kimi/Moonshot tool schemas normalize empty function parameters to a root object schema; thanks
  @jghwwnq for the provider repro (#3265).
- Novita defaults to its OpenAI-compatible `/openai/v1` endpoint so chat completions no longer
  404 out of the box; thanks @buko for the report and endpoint verification (#3255).
- Dependency security: `ws` pinned to 8.21.0 across npm packages to close remote memory-exhaustion
  DoS (dependabot).

### Community contributions

- Non-DeepSeek model pricing — thanks @mvanhorn (#3201)
- Telegram polling transport — thanks @cyq1017 (#3195)
- Mobile event history — thanks @RobertEmprechtinger (#3220)
- Runtime-API session save — thanks @gaord (#3199)
- Whale-accent rename — thanks @nightt5879 (#3197)
- `DEEPSEEK_BASE_URL` / `MODEL` honored in `exec` — thanks @hongchen1993 (#3221)
- VS Code read-only API documentation — thanks @cyq1017 (#3013)
- Atomic ask-only permission rule persistence — thanks @greyfreedom (#3233)
- DeepInfra provider support and release-surface follow-through — thanks @idling11 (#3235, closes #3231) and @nightt5879 (#3236)
- Editable oversized paste composer flow — thanks @idling11 (#3267, closes #3263)
- WeChat bridge (`integrations/weixin-bridge` via Feishu + Tencent OpenClaw) — thanks @VincentCorleone (#3206)
- Config robustness: atomic permission-rule save, one-time config `.bak` backup before the first changed write, `HELPOFAI_HOME` as primary config home, and accepting the dispatcher-written config shape (camelCase aliases + `[features.enabled]` table) so legacy/dual-written configs parse cleanly
- Dependency/CI bumps: docker login/qemu actions, softprops gh-release, download-artifact, vitest, @opennextjs/cloudflare, form-data, js-yaml, dompurify, ws

## [0.8.60] - 2026-06-13

### Added

- **Agent Fleet real-run cutover (#3154/#3096).** `helpofai fleet run` now
  launches durable workers through the headless `helpofai exec --output-format
  stream-json` path instead of the local simulation interpreter, with terminal
  worker events freeing leases so queued fleet tasks continue running.
- **Read-only shell parallelism (#2983).** The engine can now run conservative
  read-only shell calls in parallel, including strict `bash`/`sh`/`zsh -c`
  wrappers for whitelisted commands, while writes, stdin, background TTY work,
  redirects, pipes, command substitution, and follow-mode tails stay serial.
- **Declarative JS/TS HelpFlow authoring (#3097).** HelpFlow now accepts a
  compile-only `workflow({...})` JavaScript/TypeScript authoring form that
  lowers into the existing `WorkflowSpec` validator without executing user
  JavaScript.
- **Slash-menu Ctrl+P/Ctrl+N navigation (#3196).** The slash command menu now
  supports Ctrl+P/Ctrl+N movement without letting the global file picker steal
  focus while the menu is open. Thanks @1Git2Clone for the PR.
- **New models and first-party provider routes.** This release adds
  **GLM-5.2** (selectable on the Z.ai Coding Plan and over OpenRouter as
  `z-ai/glm-5.2`, alongside the existing GLM-5.1 default), a first-party
  **Z.ai** provider route, a first-party **StepFun / StepFlash** route
  (`step-3.7-flash`), and a first-party **MiniMax** route defaulting to
  `MiniMax-M3` with the M2.7/M2.5/M2.1 family selectable (#3187/#3191).

### Changed

- **README and contributor credits.** The README now has a shorter public
  overview and moves the full contributor ledger to `docs/CONTRIBUTORS.md`,
  preserving public thanks for [DeepSeek](https://github.com/deepseek-ai),
  [DataWhale](https://github.com/datawhalechina),
  [OpenWarp](https://github.com/zerx-lab/warp), and
  [Open Design](https://github.com/nexu-io/open-design).
- **Fleet-backed sub-agent direction.** Runtime docs now state the intended
  cutover clearly: "sub-agent" is role/UX vocabulary, while durable detached
  work should converge on the fleet-backed worker lifecycle with retries,
  receipts, and ledgered inspection.

### Fixed

- **Sub-agent eval no longer blocks by default.** `agent_eval` now returns the
  current projection immediately and delivers follow-up input without waiting
  for a running child to finish its provider call. Pass `block:true` for an
  intentional terminal wait.
- **Z.ai GLM thinking traces.** Direct Z.ai requests now use the documented
  `thinking` shape, preserve and replay `reasoning_content`, classify GLM
  reasoning streams as thinking output, and accept `ultracode` as a max-effort
  alias.
- **Claude skill archive compatibility (#2743).** `/skill install` keeps
  portable Claude-style skill folders supported while rejecting multi-skill
  Claude plugin archives clearly instead of silently installing only one skill
  and dropping plugin semantics. Thanks @AiurArtanis for the ecosystem request.

## [0.8.59] - 2026-06-12

### Added

- **Moonshot Kimi K2.7 Code model.** The Moonshot/Kimi provider now defaults to
  `kimi-k2.7-code`, recognizes `kimi`/`kimi-k2` aliases for that model, keeps
  explicit `kimi-k2.6` selectable, and adds the OpenRouter
  `moonshotai/kimi-k2.7-code` registry row.
- **Concise verbosity mode (#3052).** CLI noninteractive launches now default
  to concise prompt/output discipline unless overridden by config, env, or
  `--verbosity`, while interactive TUI launches remain normal by default.
  Thanks @cyq1017 for the PR.
- **Ephemeral generated project context (#3058).** Opening HelpOfAi in a
  directory with no instruction files now keeps the bounded generated project
  overview in memory instead of creating `.helpofai/instructions.md`.
- **ACP registry auth metadata (#1447).** The ACP stdio adapter now advertises
  terminal authentication setup in `initialize.authMethods`, matching the
  registry's validation requirement.
- **Sidebar context menus (#3065).** Right-clicking the sidebar no longer shows
  `Paste`; clickable sidebar rows now offer their row command as the first
  context action.
- **Sidebar hover popovers (#3088).** Streaming turns now keep sidebar hover
  popovers responsive while continuing to throttle transcript/body mouse
  motion.
- **Dark-theme selection contrast (#3074, thanks @drpars).** Session, config,
  help, context-menu, and approval selections now use the muted selection
  background instead of the bright accent color.
- **Cursor-style activity metadata rows (#3146).** Dense successful tool-run
  summaries now render as a single muted `Explored ...` / `Updated metadata`
  row, include short command-family labels for successful generic verifier
  groups, and keep keyboard/mouse expansion and detail inspection intact.
- **Provider-wait observability (#3095).** Footer stall reasons now name the
  active provider/model route, idle seconds vs stream budget, and whether a
  fanout plan is still at `0 running` or dispatch is pending. Structured
  provider-wait incidents log once per turn from the main tick loop (not on
  every footer redraw).
- **Interactive fanout launch gate (#3095).** Direct sub-agent children queue
  behind a configurable semaphore (`[subagents] interactive_max_launch`,
  default 4) with a visible `queued: waiting for an interactive fanout slot`
  reason before their first model step.
- **Goal lifecycle controls.** `/goal` is now the primary command surface for
  session goals, with `pause`, `resume`, `complete`, `blocked`, and `clear`
  controls while `/hunt` remains a compatibility alias.
- **Persistent thread-goal API.** App-server clients can now set, get, and clear
  durable thread goals through `thread/goal/set`, `thread/goal/get`, and
  `thread/goal/clear`, backed by the state store with Codex-style status and
  token/time accounting fields.
- **Command-boundary ownership layers (#2888/#3055).** Built-in slash command
  metadata now lives in `commands/registry.rs`, slash parsing in
  `commands/parse.rs`, and handlers under group-owned command areas, preserving
  the existing dispatch surface while reducing future `commands/mod.rs` churn.
- **Approval-rule source metadata (#1186/#2971).** Runtime API
  `approval.required` events now include optional `matched_rule` metadata when
  an execution-policy rule caused the prompt. Thanks @greyfreedom for the PR
  and @Ram9199 for the audit-semantics discussion.
- **Localized tool-family labels (#2901).** Tool activity labels for read,
  patch, run, find, delegate, fanout, RLM, verify, think, and generic tool
  work now route through the shipped locale tables. Thanks @gordonlu for the
  PR.
- **Localized config section labels (#2918).** The interactive config view now
  localizes section and session/saved scope labels while preserving English
  search terms. Thanks @gordonlu for the PR.
- **Localized config editor labels (#2919).** The config editor modal now
  localizes edit labels, default/unavailable placeholders, and effective
  currency hints. Thanks @gordonlu for the PR.
- **Hotbar number-key dispatch (#3056).** Bare `1`-`8` now trigger bound
  hotbar slots only when the composer is empty, while `Alt+1`-`Alt+8` trigger
  slots regardless of composer text and overlays keep key ownership. Thanks
  @reidliu41 for the PR.
- **Voice dictation commands (#3051).** `/voice`, `/voice-send`, and
  `/voice-control` now record through `sox`/`rec`/`arecord`, transcribe via the
  active provider's chat-completions API, and insert transcripts at the
  composer cursor. The `voice.toggle` hotbar action dispatches the real voice
  command, with help and status text localized across all seven shipped
  locales. Thanks @huqiantao for the PR.
- **Thread rewind and snapshot restore API (#2808).** GUI clients can now call
  `POST /v1/threads/{id}/undo`, `/patch-undo`, and `/retry` to fork, roll back,
  or rerun recent thread turns, plus `POST /v1/snapshots/{id}/restore` to
  restore a workspace snapshot by id. Thanks @bengao168 for the PR.
- **Active provider fallback chain (#2773).** Configured `fallback_providers`
  now build an ordered primary-plus-fallback route that the TUI can report,
  advance through, and reset with `/provider fallback reset`, including footer
  visibility for fallback state. Thanks @idling11 for the PR.
- **Provider metadata registry (#3005).** Built-in provider ids, display names,
  defaults, env vars, config keys, aliases, and wire formats now live in a
  shared metadata registry, with the provider drift check covering the registry
  contract. Thanks @sximelon for the PR.
- **Hugging Face provider route (#2879).** Hugging Face Inference Providers now
  have first-class config, env, docs, and registry coverage for the
  OpenAI-compatible router, including `huggingface`/`hugging-face`/
  `hugging_face`/`hf` aliases and `HUGGINGFACE_*`/`HF_*` env fallbacks. Thanks
  @mvanhorn for the PR.

### Fixed

- **SSE data lines without spaces (#3152).** Chat Completions, Responses, and
  Anthropic stream readers now accept both `data: {...}` and `data:{...}` SSE
  frames, matching the spec and preventing providers that omit the optional
  space from streaming empty output. Thanks @wgeeker for the PR.
- **Runtime thread detail N+1 reads (#3141).** `get_thread_detail` now scans
  persisted turn items once and groups them by turn instead of reading the
  items directory once per turn, preserving item order while keeping large
  thread detail loads responsive.
- **Project-local hook trust boundary (#3140).** `.helpofai/hooks.toml` is now
  loaded only after the workspace is trusted in user-owned config, matching the
  project-local MCP trust model while preserving the documented shell-command
  hook contract.
- **Skill registry sync latency (#3139).** `/skills sync` now syncs registry
  entries with bounded ordered concurrency, so network latency no longer stacks
  one skill at a time while output order stays deterministic.
- **SiliconFlow China provider config (#2893/#2895).** `siliconflow-CN`
  now reads its own `[providers.siliconflow_cn]` / `[providers.siliconflow-CN]`
  table and falls back to `[providers.siliconflow]` only for unset
  `api_key`/`base_url`/`model` fields. Thanks @Artenx for the report and
  @idling11 for the PR.
- **Self-update download timeout (#3006).** `helpofai update` now applies a
  five-minute HTTP client timeout so blocked or very slow GitHub release
  downloads fail instead of hanging indefinitely. Thanks @New2Niu for the PR.
- **Legacy `deepseek` update migration (#2960/#3013/#3053).** Running
  `deepseek update` or `deepseek-tui update` from a pre-rebrand install now
  returns copy-pasteable npm, Cargo, Homebrew, and manual-binary migration
  steps instead of trying to spawn a missing `helpofai` binary. README and
  rebrand docs now cover the same upgrade path. Thanks @jazzi and
  @tiangangQiu for the reports, @cyq1017 for the update-path PR, and
  @angus-guo for the README PR.
- **Short `hoa` shim delegation.** The `hoa` convenience binary now
  prefers the sibling `helpofai` dispatcher installed next to it before
  falling back to `PATH`, preventing fresh local builds or installs from
  accidentally invoking an older global dispatcher.
- **Constitution trust wording (#2950/#3008).** The base prompt now explains
  that "begins with an A" means a baseline of trust, not a literal output
  formatting rule. Thanks @cyq1017 for the PR.
- **TUI provider-source recovery (#3007/#3011).** Unsupported interactive
  providers now report whether the value came from `--provider`, environment,
  or config. Config-sourced unsupported providers fall back to DeepSeek without
  forwarding stale keyring secrets. Thanks @cyq1017 for the PR.
- **Exec auto-model handoff (#3148).** `helpofai exec --model auto` now
  survives the CLI/TUI boundary by honoring the HelpOfAi model env alias and
  legacy DeepSeek model handoff before falling back to provider defaults.
  Thanks @hongchen1993 for the PR.
- **macOS shortcut modifiers (#2938/#2943).** Ctrl-like shortcuts that are
  reported as `SUPER` by macOS terminals now work for backgrounding tasks and
  sidebar-focus chords without rewriting clipboard shortcuts. Thanks @idling11
  for the PR.
- **TUI mouse-report leak (#3063/#3067).** Strip raw SGR mouse coordinate
  tails from the composer even when `use_mouse_capture` is false, covering
  orphaned terminal reporting state after crashes or focus races.
- **Interrupted sub-agent lifecycle (#3080).** API-timeout interruptions now
  emit `MailboxMessage::Interrupted`, render terminal interrupted cards, and
  reconcile stale running fanout counts from manager snapshots.
- **OpenAI Codex stream diagnostics and active tool collapse (#3146).** The
  Responses bridge now reports nested `response.failed` /
  `response.incomplete` errors instead of `unknown`, and dense successful
  in-flight tool bursts collapse into the same calm activity metadata row as
  committed history.
- **OpenAI Codex reasoning tiers.** Switching from DeepSeek to `openai-codex`
  now normalizes stale reasoning state into Responses-compatible
  `low`/`medium`/`high`/`xhigh` tiers. Startup, `/config`, and the model
  picker now display Codex labels instead of leaking DeepSeek
  `off`/`max` names, while Codex still reports as a Responses payload
  provider. The Responses request builder also clamps legacy `minimal` input
  to `low` and has regression coverage that Codex requests use
  `reasoning.effort`, not DeepSeek `thinking` fields.
- **OpenAI Codex context metadata (#3070).** The `gpt-5.5` default and
  HelpOfAi aliases now use OpenAI's documented 1,050,000-token context window
  and 128,000 max-output metadata for context pressure, prompts, and doctor
  capability output.
- **OpenAI Codex effective context budgeting.** The public OpenAI API metadata
  for `gpt-5.5` remains 1,050,000 tokens, but the `openai-codex` OAuth route now
  budgets prompts against the 400K Codex-family effective window so preflight
  compaction runs before the backend returns `context_length_exceeded`.
- **OpenRouter Nemotron 3 Ultra preset.** The OpenRouter preset and model
  registry now emit `nvidia/nemotron-3-ultra-550b-a55b` while keeping the old
  Ultra aliases compatible.
- **OpenRouter auth after MiMo switches (#3064).** Switching from Xiaomi MiMo
  to OpenRouter now has regression coverage for preflight key failures and
  Bearer auth header isolation before any request can be dispatched.
- **Responses strict-tool schema compatibility (#3062/#3017/#1883).** Responses
  function tools now preserve per-tool strict-mode compatibility, keep optional
  strict-schema fields nullable, and append deterministic constraint notes when
  root composition groups must be flattened for Responses.
- **Runtime prompt autonomous loop guard (#3061).** Runtime policy reference
  now explicitly forbids initiating new work when `<runtime_prompt>` is the
  only new turn content and no tool/sub-agent handoff is pending.
- **Goal runtime status sync.** Goal token budgets and active/paused/complete
  status now sync into the engine alongside the objective, and model-visible
  `update_goal` can only mark goals complete or blocked.

### Contributors

- Devin session work on #3080/#3095 (PRs #3103, #3104, #3106) — Hunter Bown
  (maintainer integration/cherry-pick on `codex/v0.8.59-release-ready`).
- Nightt (@nightt5879) for the Responses strict-tool schema hardening in PR
  #3062.
- yekern (@yekern) for the #3061 runtime-prompt loop safety report and repro
  that shaped the dispatch guard.
- Paulo Aboim Pinto (@aboimpinto) for the staged command-boundary design and
  Layer 3 registry/parser extraction in PR #2888, plus the #2851/#2791/#2870
  architecture stream that guided the grouped command areas in #3055.

## [0.8.58] - 2026-06-11

### Added

- **Native Anthropic provider.** A dedicated Messages API adapter
  (`/v1/messages` with `x-api-key` auth) replaces OpenAI-dialect shims for
  Claude models: adaptive thinking with `output_config.effort` shaping,
  prompt-cache breakpoints (capped at 4, earliest dropped), signed-thinking
  replay via `signature_delta`, normalized cache-hit/miss usage telemetry,
  and SSE error envelopes. `claude-opus-4-8`, `claude-sonnet-4-6`, and
  `claude-haiku-4-5` join the model registry; configure with
  `ANTHROPIC_API_KEY` (#3014).
- **Hooks v2.** `tool_call_before` hooks can now return a JSON decision —
  `{"decision": "allow"|"deny"|"ask", "reason", "updatedInput",
  "additionalContext"}` — with deny > ask > allow precedence across multiple
  hooks, last-writer-wins input rewriting, and concatenated context. Exit
  code 2 remains a legacy hard deny. Hooks support glob matchers and
  project-local `.helpofai/hooks.toml` (#3026).
- **Clickable sidebar.** Background-job rows show/cancel on click, the
  Ctrl+K hint row runs `/jobs cancel-all`, and agent rows open `/subagents`;
  row actions are built in the same pass as the rendered lines so a click
  can never target the wrong job (#3028).
- OSC 8 out-of-band hyperlink infrastructure with per-region open/close
  sequences that survive partial redraws (#3029).
- `helpofai exec` gains `--allowed-tools`, `--disallowed-tools` (deny wins),
  `--max-turns`, and `--append-system-prompt` (#3027).
- Constitution prompt source: YAML source-of-truth plus Python renderer for
  the system prompt, with the active prompt now served from
  `constitution.md` (#3015, renderer reconciliation still tracked).
- Agent-task issue template, labels, and runner protocol (#3021); remote
  smoke-test droplet loop hardening — gh CLI, swapfile, agent sessions
  (#3022).

### Changed

- **Sub-agent routing is provider-aware.** DeepSeek ids are no longer
  hardcoded into model validation; routing works from per-provider
  big/cheap candidates, the network router is skipped when a provider has
  no cheap tier, and spawn-time model requests are validated against the
  active provider (#3018).
- Model-specific facts in the system prompt (context window, sub-agent
  pricing, thinking notes, architecture characteristics) are now templated
  per-model instead of hardcoded DeepSeek V4 claims, in both `base.md` and
  `constitution.md` (#3025).
- Provider capability lookups for Moonshot/OpenAI/Atlascloud resolve from
  per-model registry rows (bare and vendor-prefixed ids) instead of
  hardcoded 64K-era floors (#3023).
- Reasoning-effort now reaches Atlascloud (DeepSeek dialect), Moonshot
  (`thinking` enable/disable), and Ollama (`think` param) (#3024); Moonshot/
  Kimi models joined the reasoning-content provider and model gates (#3016).
- Transcript polish: compact tool-call cells without boilerplate (#3031),
  internal turn/agent ids hidden behind stable labels (#3030), and Ctrl+B
  now backgrounds the running foreground shell directly instead of opening
  a menu (#3032).
- The Tasks sidebar separates "Model reasoning" from "Background commands",
  and `auth list` reports the same active-credential source as
  `auth status` for openai-codex.

### Fixed

- **TUI freeze under sub-agent load.** Rapid `AgentProgress` events
  saturated the render loop and starved terminal input; progress-driven
  repaints are now throttled to one per 100ms (#3033).
- **Hooks on Windows.** Hook commands were passed to `cmd /C` through
  CRT-style argument quoting, which injected literal `\"` sequences that
  cmd.exe never unescapes — JSON decisions could not parse. Commands now
  reach cmd.exe verbatim via `raw_arg`.
- Codex Responses: assistant tool results are converted to
  `function_call_output` items (multi-turn tool calling previously broke),
  tool schemas are sanitized for the Responses API, and `maximum` effort
  maps to `xhigh` (#3019, #3017 — both partially; retry/backoff and
  per-tool strict mode remain open).
- Better tool-denial and provider error messages harvested from PR #2933
  (#3020).


## [0.8.57] - 2026-06-10

### Added

- **Turns now survive system sleep.** When the host suspends mid-stream, the
  connection used to die on wake with `Stream read error: error decoding
  response body` and the turn was lost (#2990). The engine now stamps stream
  progress with both monotonic and wall-clock time; a large divergence on a
  stream error identifies a sleep/wake cycle, and the request is silently
  re-issued (up to the existing 3-retry budget) instead of failing the turn.
- **One-command release prep.** `./scripts/release/prepare-release.sh X.Y.Z`
  bumps the workspace version, every internal crate dependency pin, the npm
  wrapper, and the README install-tag examples, refreshes `Cargo.lock`,
  regenerates the embedded TUI changelog slice and web facts, and runs
  `check-versions.sh` — the v0.8.56 release needed nine follow-up commits for
  exactly these sync points.
- `.github/CODEOWNERS` and `.github/dependabot.yml` (weekly cargo +
  github-actions updates, monthly npm for `web/`).

### Changed

- **The changelog went on a diet.** Root `CHANGELOG.md` now carries recent
  releases (v0.8.40+); older entries moved to `docs/CHANGELOG_ARCHIVE.md`.
  `crates/tui/CHANGELOG.md` — embedded into every binary for `/change` — is a
  generated 15-release slice (`scripts/sync-changelog.sh`), no longer a
  357 KB manual byte-for-byte copy (~300 KB smaller binaries).
- GitHub Release bodies are generated from the tagged version's changelog
  section (`scripts/release/generate-release-body.sh`) instead of a
  hardcoded workflow blob with a hand-pasted contributor list.
- `check-versions.sh` now also gates `web/lib/facts.generated.ts` and the
  README install-tag examples; the CNB mirror pipeline validates the pushed
  tag against `Cargo.toml` before generating release notes.
- Docs reorganized: internal design notes moved under `docs/rfcs/`; stale
  internal docs (old audits, handoffs, region-specific VM notes) removed.
- Agent-facing polish: the system prompt environment block reports
  `helpofai_version` (was `deepseek_version`), the legacy
  `.deepseek/instructions.md` path is no longer advertised in the prompt
  (still honored for back-compat), and oversized instruction files are
  truncated with an explicit `[…truncated: N bytes omitted]` marker instead
  of a bare ellipsis.

### Fixed

- **Docker images build again.** The release `docker` job failed for v0.8.56
  because the Dockerfile still copied the pre-rebrand `deepseek` /
  `deepseek-tui` binaries; they are now symlinks to the helpofai binaries
  inside the image, so legacy container entrypoints keep working.
- `.devcontainer/devcontainer.json` used the pre-rebrand container name,
  mount path, and `deepseek` remote user.
- Stale `--bin deepseek` examples, `DeepSeek-TUI` strings in `/change`
  output, and pre-rebrand doc comments.

### Removed

- Unused dependencies: `tracing-appender` and `zeroize` (TUI crate),
  `rustls` (release crate); the orphaned `vendor/schemaui-0.12.0` lockfile
  leftover and a machine-specific one-off `scripts/verify_task.sh`.

## [0.8.56] - 2026-06-09

### Added

- **Status picker localization.** The status picker surface (7 MessageIds) is
  now localized across all supported locales (#2896, @gordonlu).
- **Approval dialog localization.** The approval dialog surface is now
  localized across 7 locales: English, Simplified Chinese, Japanese,
  Vietnamese, Portuguese, Spanish, and French (#2891, @gordonlu).
- **Volcengine provider in TUI dispatcher.** The `helpofai` / `helpofai-tui`
  CLI dispatcher now allows the Volcengine provider, so users can launch
  directly into a Volcengine-backed session (#2923, @hongchen1993).
- **Dispatcher API-key preference.** When a provider-specific API key is
  supplied via the CLI dispatcher, it is now preferred over the saved root
  key, fixing a regression where saved keys masked explicit CLI keys (#2928,
  @hongchen1993).
- **Qwen 3.6 Plus model support.** Added complete Qwen 3.6 Plus model
  resolution with dedicated version-bump tests (#2930, @idling11).
- **Oversized paste spill.** Pastes larger than ~10 KB are now written to
  `.helpofai/pastes/` instead of being truncated or dropped, preserving the
  full content for the session (#2920, @sximelon).
- **Cross-session prompt cache.** Added a disk-backed cross-session prompt
  base-section cache so post-mode-flip and post-restart turns reuse the
  byte-stable prefix without rebuilding it from scratch.

### Fixed

- **Background shell routing.** Shell commands expected to take >5 seconds are
  now automatically guided to background tasks instead of blocking the agent
  loop, with the task panel syncing immediately on cancel (#2947, #2941,
  @cyq1017, @idling11).
- **`allow_shell` error naming.** Shell-tool refusal errors now explicitly name
  `allow_shell = false` as the reason and suggest `/config allow_shell true` as
  the escape hatch (#2905, @cyq1017).
- **Prefix-cache stability across mode flips.** `allow_shell` is now decoupled
  from the static system-prompt prefix, so mode changes (Plan ↔ Agent ↔ YOLO)
  no longer rebuild the byte-stable message[0] and invalidate the DeepSeek
  prefix cache (#2949, @LeoAlex0).
- **`visibility="internal"` explained.** The Runtime Policy Reference section
  of the system prompt now explains the `visibility="internal"` attribute so
  models stop narrating their current mode between steps (#2951, @LeoAlex0).
- **Bocha web search response handling.** Updated response parsing for the
  Bocha search backend after an upstream API change (#2946, @h3c-hexin).
- **PDF read hang.** Full-PDF reads now use `extract_text_by_pages` to avoid
  a hang on large or complex PDFs (#2898, @idling11).
- **9 critical bugs.** Fixed bugs across tools, client, and commands: stale
  `ContentBlockStop` cleanup, missing `#[test]` attribute, trailing-space
  restoration on English `ApprovalField` labels, and several
  correctness/stability issues (#2880, @HUQIANTAO).

### Changed

- **CNB shim cleanup.** Removed deprecated `deepseek` shim references from the
  CNB mirror path.
- **Style.** Applied `cargo fmt` to `crates/tools/src/file.rs`.

## [0.8.55] - 2026-06-08

### Added

- **Together AI provider.** Added Together AI as a first-class provider
  (`[providers.together]`, `TOGETHER_API_KEY`/`TOGETHER_BASE_URL`/`TOGETHER_MODEL`)
  with default models `deepseek-ai/DeepSeek-V4-Pro` and
  `deepseek-ai/DeepSeek-V4-Flash`, TUI provider-picker/auth/capability support,
  and CLI `auth list`/`auth status` coverage.
- **Model catalog updates.** Added Qwen 3.7 Max (`qwen/qwen3.7-max`), MiniMax 2.7
  (`minimax/minimax-2.7`), and NVIDIA Nemotron 3 Ultra (`nvidia/nemotron-3-ultra`)
  on OpenRouter.
- **OpenAI Codex (ChatGPT) provider — experimental.** Added an `openai-codex`
  provider that reuses an existing ChatGPT/Codex CLI OAuth login. The access
  token is read and refreshed from `~/.codex/auth.json` (no API key is stored),
  and requests use the OpenAI Responses API at `/codex/responses` with the
  `chatgpt-account-id` header and `responses=experimental` beta opt-in. Env
  overrides: `OPENAI_CODEX_ACCESS_TOKEN`/`CODEX_ACCESS_TOKEN`,
  `OPENAI_CODEX_BASE_URL`/`CODEX_BASE_URL`, `OPENAI_CODEX_MODEL`/`CODEX_MODEL`,
  `OPENAI_CODEX_ACCOUNT_ID`/`CODEX_ACCOUNT_ID`, `OPENAI_CODEX_AUTH_FILE`,
  `CODEX_HOME`. Default model `gpt-5.5`. The live Responses round-trip has not
  been exercised against the production backend in CI; treat as preview.

---

Older releases: [CHANGELOG.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/CHANGELOG.md) and [docs/CHANGELOG_ARCHIVE.md](https://github.com/helpofai/HelpOfAi-Cli/blob/main/docs/CHANGELOG_ARCHIVE.md).
