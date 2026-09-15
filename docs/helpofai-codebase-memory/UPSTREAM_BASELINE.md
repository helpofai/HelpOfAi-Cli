# UPSTREAM BASELINE

**Purpose:** This document records the original state of the upstream `codebase-memory-mcp` integration, establishing a baseline to ensure future maintainers know exactly what was consumed.

- **Upstream Repository:** `https://github.com/DeusData/codebase-memory-mcp`
- **Integration Profile:** Pre-built native binary supervisor. HelpOfAi does NOT compile the C11 source.
- **Tested Upstream Version:** `latest` (as of integration date)
- **Deployment Strategy:** Platform-specific downloads via `reqwest`, securely unpacked to `%LOCALAPPDATA%\HelpOfAi\engines\codebase-memory` (Windows) or `~/.local/share/helpofai/engines/codebase-memory` (POSIX).

## Checksums and Verification

All binaries are explicitly checked against their SHA-256 releases via `checksums.txt` during the `helpofai codebase install` operation. 

## Integration Capability Matrix
- [x] **Graph UI Extraction:** Built as a standalone `GraphSupervisor` avoiding TUI thread blockage.
- [x] **Binary Execution:** Hooked via MCP and direct CLI arguments using `helpofai codebase <cmd>`.
- [x] **Version Polling:** Added natively to `install`, `update`, and `doctor`.
- [x] **Daemons Context:** Persistent memory via `CBM_CACHE_DIR` routing inside the HelpOfAi engine folder.

> *Never lose the upstream commit identity. The downloaded artifacts map directly to official Release tags at the DeusData namespace.*
