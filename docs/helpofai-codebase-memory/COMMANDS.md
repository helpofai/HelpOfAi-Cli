# HelpOfAi Codebase Memory Commands

## helpofai codebase doctor
Verifies engine health, path configurations, downloaded artifacts, and checking the persistent database graph state.

## helpofai codebase install
Securely downloads, checksums, unpacks, and configures the latest UPSTREAM binary release from DeusData without tracking upstream commits arbitrarily in Git.

## helpofai codebase update
In-place upgrade utility allowing users to step versions safely.

## helpofai codebase <subcommand>
Direct passthrough for all native codebase-memory-mcp CLI operations.

## helpofai graph start
Initiates a background UI instance of the persistent graph via a memory-isolated Subprocess map pointing uniquely to CBM_CACHE_DIR.

## helpofai graph status
Reads from the isolated Process ID tracker internally checking kill -0 or 	asklist natively to verify health without polling HTTP.

## helpofai graph stop
Graceful termination of UI-based local HTTP handlers (running on localhost:9749).
