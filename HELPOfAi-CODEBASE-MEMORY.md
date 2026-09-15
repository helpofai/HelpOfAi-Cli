# HELPOfAi CODEBASE MEMORY

## Master Engineering, Fork Integration, Rebranding & AIOS Integration Mission

You are working inside the HelpOfAi-Cli repository.

Your mission is to create a production-grade, globally reusable **HelpOfAi Codebase Memory Engine** by carefully integrating and rebranding the upstream:

`https://github.com/DeusData/codebase-memory-mcp`

Do NOT create a simplified clone.

Do NOT create a prototype.

Do NOT remove advanced functionality.

Do NOT replace mature implementations with toy implementations.

The goal is to preserve the upstream project's powerful code-intelligence capabilities while transforming the product identity, integration layer, configuration, lifecycle management, CLI experience, and AIOS integration into a first-class HelpOfAi component.

The final result must be suitable for a serious cross-platform developer tool.

---

# 0. CORE OBJECTIVE

Build:

**HelpOfAi Codebase Memory**

A local-first, high-performance code intelligence and persistent codebase knowledge-graph engine for HelpOfAi AIOS.

It must provide:

- fast repository indexing
- persistent knowledge graph
- AST-based code intelligence
- Tree-sitter language support
- symbol extraction
- relationship extraction
- call graph analysis
- type/semantic resolution
- dependency analysis
- structural search
- code search
- semantic search
- BM25/full-text search
- impact analysis
- dead-code detection
- architecture analysis
- cross-service linking
- cross-repository intelligence
- infrastructure-as-code indexing
- incremental indexing
- filesystem/git-aware watching
- persistent storage
- MCP server
- CLI access
- graph query capability
- built-in 3D graph visualization
- graph UI server
- session/daemon coordination
- HelpOfAi integration
- Windows/Linux/macOS support

The engine must remain useful even without an LLM.

The LLM/agent is a consumer of the structured code intelligence.

---

# 1. FIRST RULE — AUDIT BEFORE MODIFYING

Before changing ANY source file:

1. Inspect the entire repository.
2. Read README and all architecture documentation.
3. Inspect Cargo/build configuration.
4. Inspect source directories.
5. Inspect graph-ui.
6. Inspect vendored dependencies.
7. Inspect tests.
8. Inspect installation scripts.
9. Inspect platform-specific code.
10. Inspect MCP implementation.
11. Inspect daemon/session coordination.
12. Inspect persistence/storage.
13. Inspect indexing pipeline.
14. Inspect semantic search implementation.
15. Inspect LSP/type-resolution implementation.
16. Inspect graph visualization implementation.
17. Inspect configuration system.
18. Inspect CLI commands.
19. Inspect packaging/release infrastructure.
20. Inspect LICENSE.
21. Inspect THIRD_PARTY.md.
22. Inspect SECURITY.md.
23. Identify all upstream attribution obligations.

Do not modify anything during this audit phase.

Produce:

`docs/helpofai-codebase-memory/AUDIT.md`

containing:

- current architecture
- module map
- dependency map
- build system
- runtime model
- data flow
- indexing pipeline
- graph architecture
- UI architecture
- MCP architecture
- daemon architecture
- persistence model
- platform support
- test architecture
- third-party components
- licenses
- attribution requirements
- potential conflicts with HelpOfAi-Cli
- integration risks
- proposed migration strategy

---

# 2. CREATE AN UPSTREAM BASELINE

Before rebranding, create a reproducible baseline.

Record:

- upstream repository URL
- upstream commit SHA
- upstream version/tag if applicable
- source tree checksum where practical
- build result
- test result
- graph UI result
- MCP result
- CLI result
- indexing result
- semantic-search result

Create:

`docs/helpofai-codebase-memory/UPSTREAM_BASELINE.md`

The baseline must allow future maintainers to determine exactly which upstream implementation was incorporated.

Never lose the upstream commit identity.

---

# 3. LICENSE AND THIRD-PARTY COMPLIANCE

This is mandatory but as a AI you can not do it, i will my self after completely done and working mode.

Do NOT blindly replace:

- copyright notices
- LICENSE files
- third-party attribution
- dependency notices
- source attribution

Create a dedicated:

`THIRD_PARTY_NOTICES.md`

or an equivalent existing HelpOfAi third-party notice mechanism.

Document:
/_
|--------------------------------------------------------------------------
| HelpOfAi (HOA) Professional Software
|--------------------------------------------------------------------------
|
| Copyright (c) 2026 Rajib Adhikary. All Rights Reserved.
|
| This file is part of the HelpOfAi Professional Software Suite.
| Unauthorized copying, modification, redistribution, reverse engineering,
| decompilation, or commercial use of this source code, in whole or in part,
| is strictly prohibited without prior written permission from the copyright owner.
|
| Author : Rajib Adhikary
| Organization: HelpOfAi (HOA)
| Website : https://helpofai.com
| Location : Basta Purba Para, Aranghata, Nadia, West Bengal, India
|
| This source code contains proprietary and confidential information.
| Any unauthorized access or distribution may violate applicable copyright laws.
|
|--------------------------------------------------------------------------
_/

- DeusData/codebase-memory-mcp
- upstream license
- upstream commit/version
- modifications made by HelpOfAi
- important third-party dependencies
- their licenses
- bundled assets
- graph UI dependencies
- Tree-sitter grammars
- embedding models/assets
- other vendored code

Do not claim that all code is originally written by HelpOfAi.

Brand the PRODUCT as HelpOfAi while maintaining legally required attribution.

If an attribution requirement is unclear, STOP and report it instead of guessing.

---

# 4. DO NOT DESTROY UPSTREAM FUNCTIONALITY

The following are considered protected capabilities.

The implementation must retain or provide equivalent functionality for:

## Indexing

- full repository indexing
- incremental indexing
- changed-file detection
- git-aware change detection
- filesystem watching
- package/module discovery
- manifest scanning
- large repository handling
- memory-efficient indexing

## Parsing

- Tree-sitter AST parsing
- all upstream supported grammars
- language detection
- syntax extraction

Do not arbitrarily reduce the language count.

## Semantic resolution

Preserve the upstream semantic/type-resolution capabilities.

Do not replace a real resolver with:

- regex-only parsing
- simplistic string matching
- fake symbol resolution
- LLM guesses

## Knowledge graph

Preserve graph persistence and relationships.

Support nodes and relationships such as:

- files
- directories
- modules
- packages
- functions
- methods
- classes
- interfaces
- traits
- structs
- enums
- variables
- routes
- resources
- services
- tests
- configuration
- infrastructure resources

Preserve meaningful relationship types such as:

- CALLS
- IMPORTS
- DEFINES
- IMPLEMENTS
- INHERITS
- HTTP_CALLS
- ASYNC_CALLS
- EMITS
- LISTENS_ON
- DATA_FLOWS
- SIMILAR_TO
- SEMANTICALLY_RELATED

and all additional upstream relationships.

---

# 5. SEARCH ENGINE

Preserve the mature search architecture.

Support:

### Structural search

Queries based on:

- node types
- names
- relationships
- graph structure
- file scopes
- degree
- architecture

### Full-text search

Preserve BM25/SQLite FTS-style functionality where provided by upstream.

### Code search

Preserve graph-aware code search.

### Semantic search

Preserve the upstream semantic embedding/search architecture.

Do not replace bundled/local embeddings with an online API.

The default engine must remain local-first.

No mandatory API key.

No mandatory cloud service.

No mandatory Docker.

---

# 6. IMPACT AND ARCHITECTURE INTELLIGENCE

Preserve:

- architecture overview
- call graph
- impact analysis
- dead-code detection
- dependency analysis
- changed-code analysis
- hotspot analysis
- module/community detection
- cross-service analysis
- route analysis
- ADR support where present

These capabilities are foundational to HelpOfAi's planning and implementation engines.

---

# 7. CROSS-REPOSITORY INTELLIGENCE

Preserve upstream cross-repository capabilities.

Design it so HelpOfAi can maintain:

```text
Global Codebase Memory
        │
        ├── Repository A
        ├── Repository B
        ├── Repository C
        └── Repository N
```

The engine must support relationships across repositories without corrupting repository-local identity.

Repository identity must be explicit.

Avoid collisions between identical file paths, symbols, package names, or node IDs from different repositories.

---

# 8. INFRASTRUCTURE-AS-CODE

Preserve infrastructure indexing.

Support the upstream capabilities for:

- Dockerfiles
- Kubernetes
- Kustomize
- service/resource relationships
- infrastructure dependencies

Treat infrastructure as part of the software architecture graph.

---

# 9. PERSISTENCE

Maintain persistent local storage.

The database/index must survive:

- CLI exit
- MCP session exit
- agent restart
- computer restart

Use the upstream storage architecture where appropriate.

Do not introduce a mandatory external database server.

The default experience must remain local.

---

# 10. GLOBAL HELPOFAI STORAGE

Add a HelpOfAi-managed storage layer around the engine.

structure:
Or you can check exiting structure already helpofai-cli have:
Windows:

`%LOCALAPPDATA%/HelpOfAi/`

Linux:

`~/.local/share/helpofai/`

macOS:

`~/Library/Application Support/HelpOfAi/`

Create a stable structure such as:

```text
HelpOfAi/
├── engines/
│   └── codebase-memory/
├── codebase-memory/
│   ├── repositories/
│   ├── global/
│   ├── indexes/
│   ├── runtime/
│   └── logs/
├── config/
└── cache/
```

Do not hard-code Linux paths into Windows code.

Use platform-aware path resolution.

---

# 11. PRODUCT REBRANDING

The user-facing product should become:

**HelpOfAi Codebase Memory**

Preferred executable:

`helpofai-codebase-memory`

or another HelpOfAi-compatible executable name selected after inspecting HelpOfAi-Cli naming conventions.

Preferred CLI namespace:

`helpofai codebase`

or:

`helpofai memory`

Choose the option that best fits the existing HelpOfAi command architecture.

Do not blindly rename internal variables if doing so would make future upstream synchronization difficult.

Separate:

1. upstream-compatible internal layer
2. HelpOfAi integration layer
3. HelpOfAi branding layer

This is extremely important.

---

# 12. HELPOfAI NATIVE CLI

Add commands similar to:

```text
helpofai codebase index
helpofai codebase index <path>
helpofai codebase search
helpofai codebase semantic-search
helpofai codebase architecture
helpofai codebase impact
helpofai codebase trace
helpofai codebase status
helpofai codebase watch
helpofai codebase stop
helpofai codebase doctor
helpofai codebase graph
```

Also consider:

```text
helpofai graph
helpofai graph start
helpofai graph stop
helpofai graph restart
helpofai graph status
helpofai graph open
```

Do not duplicate functionality unnecessarily.

Reuse the codebase-memory engine.

---

# 13. 3D GRAPH VISUALIZATION

Preserve the upstream 3D graph visualization.

Do not rebuild it unless technically necessary.

The graph UI must remain independently accessible.

Preferred behavior:

```text
helpofai graph
```

starts the Codebase Memory graph service as a separate process/service and opens:

`http://127.0.0.1:<port>`

Do not embed a WebView into the HelpOfAi terminal unless explicitly required.

The HelpOfAi terminal must remain lightweight.

The browser should handle graph rendering.

---

# 14. GRAPH PROCESS MANAGER

Create a HelpOfAi lifecycle manager.

Responsibilities:

- locate graph engine
- verify installation
- verify UI support
- choose port
- start process
- monitor process
- health check
- stop process
- restart process
- open browser
- detect stale PID
- handle port conflicts
- write logs
- clean stale runtime state

Example runtime state:

```text
HelpOfAi/
└── codebase-memory/
    └── runtime/
        └── graph.json
```

Store:

- PID
- port
- version
- executable path
- start time
- project
- status

Never trust a stale PID file.

Verify the actual process.

---

# 15. GRAPH UI SECURITY

Bind the graph server to loopback by default:

`127.0.0.1`

Do not expose the graph UI to:

`0.0.0.0`

unless the user explicitly requests network access.

Never make the local graph server remotely accessible by default.

---

# 16. MCP INTEGRATION

Preserve the upstream MCP server.

HelpOfAi should expose Codebase Memory through its own MCP integration layer where appropriate.

The agent should be able to request structured intelligence such as:

- architecture
- symbol search
- semantic search
- graph search
- call tracing
- impact analysis
- dead code
- cross-service relationships
- repository status
- indexed coverage

Do not make the MCP layer return huge raw files unnecessarily.

The purpose is to reduce context consumption.

---

# 17. AIOS INTEGRATION

Integrate Codebase Memory into:

### Analysis Engine

Use Codebase Memory to understand:

- architecture
- dependencies
- symbols
- call paths
- modules
- existing patterns

### Planning Engine

Use it to determine:

- affected files
- affected symbols
- dependency impact
- test impact
- architecture boundaries
- implementation risks

### Implementation Engine

Use targeted graph queries before editing.

Do not allow the agent to blindly edit files without understanding relevant relationships when graph information is available.

### Verification Engine

Use the graph to verify:

- callers
- implementations
- references
- affected routes
- affected tests
- stale relationships

---

# 18. CONTEXT OPTIMIZATION

The engine exists partly to reduce LLM context usage.

Do not return entire repositories to the LLM.

Prefer:

```text
User request
    ↓
Codebase query
    ↓
Structured graph result
    ↓
Relevant source excerpts
    ↓
LLM
```

rather than:

```text
User request
    ↓
read 50 files
    ↓
send everything to LLM
```

Preserve the structural-query-first philosophy.

---

# 19. HELPOfAI GLOBAL CODEBASE MEMORY

Implement a global repository registry.

Example conceptual model:

```text
Global Memory
│
├── Repository
│   ├── identity
│   ├── path
│   ├── git remote
│   ├── branch
│   ├── commit
│   └── graph
│
├── Repository
│
└── Repository
```

Support:

```text
helpofai codebase list
helpofai codebase register <path>
helpofai codebase unregister <path>
helpofai codebase status
```

Do not accidentally index unrelated directories.

Explicitly identify repository roots.

---

# 20. WATCHER ARCHITECTURE

Preserve the upstream watcher.

Ensure:

- file changes are detected
- graph updates are incremental
- deleted files are removed
- renamed files are handled
- git changes are handled
- stale symbols are removed
- relationships are updated
- indexing does not consume excessive CPU continuously

Avoid full re-indexing for every keystroke.

Use batching/debouncing where appropriate.

---

# 21. PERFORMANCE REQUIREMENTS

Preserve upstream performance characteristics where practical.

Do not introduce:

- unnecessary network calls
- cloud dependencies
- LLM calls during indexing
- blocking UI operations
- unnecessary file rereads
- full repository re-indexing after every change

Benchmark:

- small repo
- medium repo
- large repo
- multi-repository workspace

Record results.

Create:

`docs/helpofai-codebase-memory/PERFORMANCE.md`

Never fabricate benchmark numbers.

Only report measured values.

---

# 22. MEMORY REQUIREMENTS

The engine must be practical on ordinary developer machines.

Use:

- bounded caches
- incremental processing
- compressed intermediate data where upstream already uses it
- memory release after indexing
- lazy loading where practical

Do not load every source file permanently into RAM.

---

# 23. WINDOWS / LINUX / MACOS

The final system must support:

Windows x64
Linux x64
Linux ARM64 where upstream/build architecture permits
macOS x64
macOS ARM64

Use Rust platform abstractions.

Do not scatter platform checks throughout business logic.

Create a dedicated platform layer.

---

# 24. INSTALLATION

HelpOfAi should eventually provide:

```text
helpofai codebase install
helpofai codebase update
helpofai codebase doctor
```

The installer must:

1. detect platform
2. detect architecture
3. install correct engine
4. verify integrity
5. verify executable
6. verify graph UI
7. verify MCP
8. initialize storage
9. perform a smoke test

Do not download arbitrary binaries from unknown URLs.

Only use explicitly trusted release sources.

---

# 25. UPDATE SYSTEM

Do not make the running executable overwrite itself.

Use a supervisor/updater process.

Support safe replacement.

Maintain rollback capability.

Never destroy a working version before the new version passes validation.

---

# 26. TESTING

Do not stop after compilation.

Run:

- unit tests
- integration tests
- parser tests
- indexing tests
- graph tests
- search tests
- semantic tests
- MCP tests
- CLI tests
- watcher tests
- persistence tests
- cross-repository tests
- graph UI smoke test
- Windows build
- Linux build
- macOS build where available

Create automated regression tests for every modified upstream subsystem.

---

# 27. UPSTREAM REGRESSION TEST

Create a baseline test suite that proves HelpOfAi modifications did not remove upstream functionality.

At minimum verify:

```text
index repository
search symbol
search code
semantic search
trace calls
architecture
impact analysis
dead code
cross repo
MCP
graph UI
watcher
persistent storage
CLI mode
```

---

# 28. HELPOfAI BRANDING

Replace user-facing branding carefully.

Change:

- product name
- CLI help
- documentation branding
- graph page branding where appropriate
- logs
- configuration labels
- installation messages

Do NOT modify legally required attribution.

Do NOT falsely claim upstream technology was originally written by HelpOfAi.

Use wording such as:

"HelpOfAi Codebase Memory"

and in third-party notices:

"Based on and incorporating code from DeusData/codebase-memory-mcp under the applicable open-source license."

Only use that wording if accurate to the final implementation.

---

# 29. DO NOT BREAK UPSTREAM MERGEABILITY

Keep upstream-derived code logically separated where practical.

Preferred:

```text
upstream-compatible/
helpofai-integration/
helpofai-ui/
helpofai-runtime/
```

Avoid unnecessary edits to upstream algorithms.

When changing upstream code is necessary:

- document why
- record file
- record reason
- record behavioral change
- add tests

Create:

`docs/helpofai-codebase-memory/MODIFICATIONS.md`

with:

```text
File
Original behavior
HelpOfAi modification
Reason
Risk
Tests
```

---

# 30. NO FAKE FEATURES

Never implement a placeholder that pretends to be functional.

Forbidden:

```text
TODO: implement later
fake graph
mock semantic search
fake LSP resolver
dummy MCP result
hardcoded architecture
hardcoded language list
fake benchmark
fake health status
```

If a feature cannot be completed correctly:

1. identify the blocker
2. report it
3. do not fake it

---

# 31. DOCUMENTATION

Create complete documentation:

```text
docs/helpofai-codebase-memory/
├── AUDIT.md
├── UPSTREAM_BASELINE.md
├── ARCHITECTURE.md
├── INSTALLATION.md
├── CONFIGURATION.md
├── CLI.md
├── MCP.md
├── GRAPH.md
├── INDEXING.md
├── SEARCH.md
├── SEMANTIC_SEARCH.md
├── IMPACT_ANALYSIS.md
├── CROSS_REPOSITORY.md
├── PERFORMANCE.md
├── SECURITY.md
├── MODIFICATIONS.md
├── THIRD_PARTY.md
└── TROUBLESHOOTING.md
```

---

# 32. SECURITY

Review all code that:

- reads files
- executes processes
- writes configuration
- starts HTTP servers
- downloads binaries
- updates binaries
- accesses Git
- handles MCP messages

Default graph binding must be local-only.

Validate paths.

Prevent directory traversal.

Avoid arbitrary command execution.

Do not trust repository contents as executable instructions.

Do not allow indexed code to automatically become shell commands.

---

# 33. MCP SECURITY

MCP stdout must remain clean JSON-RPC.

Never print logging/debug information into MCP stdout.

Use stderr or dedicated logs.

Validate all MCP arguments.

Bound expensive queries.

Prevent unrestricted filesystem access outside authorized repository roots.

---

# 34. AI AGENT SAFETY

Codebase Memory is an information system.

It must NOT automatically execute code merely because source code contains:

- shell commands
- scripts
- package hooks
- build instructions
- malicious comments
- prompt injection text

Treat repository contents as untrusted data.

---

# 35. GRAPH UI PROCESS MODEL

The graph UI must be independently launchable.

Desired behavior:

```text
helpofai graph
```

must:

1. find Codebase Memory engine
2. check whether graph service is already running
3. reuse existing service if healthy
4. otherwise start it
5. wait for readiness
6. open browser
7. return control to the terminal

Do not block the terminal indefinitely.

Support:

```text
helpofai graph start
helpofai graph stop
helpofai graph restart
helpofai graph status
helpofai graph open
```

---

# 36. DO NOT DUPLICATE THE UPSTREAM DAEMON

Before creating a new HelpOfAi daemon, determine whether the upstream coordination daemon already provides the required functionality.

If it does:

- integrate with it
- do not create a competing daemon

Only create a HelpOfAi supervisor where it provides functionality the upstream system does not.

Avoid:

```text
daemon A
daemon B
watcher A
watcher B
indexer A
indexer B
```

performing the same job.

---

# 37. CONFIGURATION

Expose HelpOfAi configuration cleanly.

Example conceptual configuration:

```toml
[codebase_memory]
enabled = true
auto_index = true
auto_watch = true
semantic_search = true
cross_repository = true
graph_ui = true

[codebase_memory.graph]
host = "127.0.0.1"
port = 9749
auto_open = true
```

Adapt this to the existing HelpOfAi configuration architecture instead of blindly introducing another configuration system.

---

# 38. LOGGING

Use structured logs.

Categories:

```text
INDEX
GRAPH
SEARCH
MCP
WATCHER
UI
DAEMON
SECURITY
UPDATE
ERROR
```

Avoid noisy logs during normal operation.

---

# 39. ERROR HANDLING

Every failure must produce a useful diagnostic.

Bad:

```text
Error
```

Good:

```text
Codebase Memory graph could not start.

Reason:
Port 9749 is already occupied.

Suggested actions:
1. Run `helpofai graph status`
2. Run `helpofai graph --port 9750`
3. Stop the conflicting process
```

---

# 40. FINAL VALIDATION

Before declaring the work complete, run a complete validation.

Verify:

### Build

```text
cargo check
cargo build
```

### Tests

```text
cargo test
```

plus all project-specific tests.

### CLI

```text
helpofai --help
helpofai codebase --help
helpofai graph --help
```

### Index

Index a real repository.

### Search

Test:

- symbol
- code
- structural
- semantic

### Graph

Verify:

```text
helpofai graph
```

opens the actual 3D graph.

### MCP

Verify MCP protocol behavior.

### Persistence

Restart the application and confirm the graph remains available.

### Watcher

Modify a source file and verify the graph updates.

### Cross-repository

Index two repositories and verify repository identity and cross-repo relationships.

---

# 41. FINAL REPORT

At the end create:

`docs/helpofai-codebase-memory/IMPLEMENTATION_REPORT.md`

Include:

1. What was integrated
2. Upstream commit/version
3. Files changed
4. Files added
5. Features preserved
6. Features modified
7. HelpOfAi additions
8. License handling
9. Third-party dependencies
10. Tests executed
11. Test results
12. Performance measurements
13. Known limitations
14. Security review
15. Future upstream merge strategy

Do not claim completion unless the implementation has actually been tested.

---

# 42. EXECUTION ORDER

Work in this exact order:

PHASE 1 — Repository discovery
PHASE 2 — Upstream audit
PHASE 3 — License/third-party audit
PHASE 4 — Baseline build
PHASE 5 — Baseline tests
PHASE 6 — Integration architecture
PHASE 7 — Codebase Memory integration
PHASE 8 — HelpOfAi runtime adapter
PHASE 9 — CLI integration
PHASE 10 — Global repository registry
PHASE 11 — Graph process manager
PHASE 12 — 3D graph integration
PHASE 13 — MCP integration
PHASE 14 — AIOS Analysis integration
PHASE 15 — AIOS Planning integration
PHASE 16 — AIOS Implementation integration
PHASE 17 — Watcher/incremental integration
PHASE 18 — Cross-repository integration
PHASE 19 — Security hardening
PHASE 20 — Cross-platform validation
PHASE 21 — Documentation
PHASE 22 — Regression testing
PHASE 23 — Final audit
PHASE 24 — Implementation report

Do NOT skip phases.

Do NOT implement everything in one uncontrolled edit.

After every major phase:

1. inspect changes
2. compile
3. run relevant tests
4. inspect output
5. fix regressions
6. document the result

---

# 43. AGENT OPERATING RULES

You are an engineering agent, not a text generator.

Before modifying code:

- inspect
- understand
- plan
- modify
- test
- verify

Never:

- overwrite files blindly
- delete unfamiliar code
- replace working systems with simplified versions
- remove tests to make builds pass
- suppress compiler errors
- hide warnings
- fake successful tests
- claim a feature works without executing it

When uncertain, inspect more source code.

When a dependency is unclear, inspect its manifest and documentation.

When a license is unclear, stop and report it.

When an upstream feature is complex, preserve it instead of rewriting it unnecessarily.

---

# 44. SUCCESS CRITERIA

The mission is complete only when HelpOfAi provides a genuine Codebase Memory capability comparable to the upstream system while being integrated into HelpOfAi AIOS.

The finished product should conceptually provide:

```text
                 HelpOfAi AIOS
                       │
                       ▼
             Codebase Memory Engine
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
      Parser         Graph          Search
        │              │              │
        ▼              ▼              ▼
       AST        Knowledge Graph   Semantic
        │              │              │
        └──────────────┼──────────────┘
                       ▼
              Architecture Intelligence
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       Planning    Implementation Verification
                       │
                       ▼
                HelpOfAi Agent
```

The result must feel like a **native HelpOfAi AIOS subsystem**, not a random third-party binary glued onto the project.

At the same time, preserve the upstream implementation's mature functionality wherever possible rather than rebuilding it unnecessarily.

END OF MASTER MISSION.
