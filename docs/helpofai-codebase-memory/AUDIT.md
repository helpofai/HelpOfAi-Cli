# Codebase Memory Upstream Audit

## Architecture Overview
The upstream architecture is a single statically compiled C11 binary (`codebase-memory-mcp`), utilizing Tree-sitter for AST generation, SQLite for persistent graph storage, and Nomic vector models for semantic embeddings.

## Module Map
- **Cache/Storage Layer:** Driven via `CBM_CACHE_DIR` routing persistence paths.
- **MCP Provider:** Stdio JSON-RPC interface for AI consumption.
- **Graph UI:** Attached HTTP worker dynamically rendering graph states on localhost.

## Build System & Packaging
Upstream uses a localized `Makefile.cbm`.
To maintain platform stability across Windows/Linux/macOS without bleeding C dependencies into the pure-Rust HelpOfAi workspace, we employ an **Embedded Artifact Delivery Strategy**. The upstream CI handles cross-compilation (Windows MSVC, macOS Mach-O, Linux ELF), and HelpOfAi's `installer.rs` securely manages checksum-verified artifact downloading directly from canonical Releases.

## Proposed Migration Strategy
- [x] Phase 1: Build `helpofai codebase install` layer natively to pull artifacts.
- [x] Phase 2: Build `CodebaseMemoryManager` bounding lifecycle execution safely off-thread UI.
- [x] Phase 3: Wire `helpofai graph` supervisor hook for UI isolation.
- [x] Phase 4: Integrate AIOS Engine specs pointing to the memory graph.
