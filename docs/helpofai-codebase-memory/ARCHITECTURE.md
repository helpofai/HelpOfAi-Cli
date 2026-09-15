# HelpOfAi Codebase Memory Architecture Layer

## Abstraction
We cleanly separate:
1. upstream-compatible internal layer (codebase-memory-mcp binary process management)
2. HelpOfAi integration layer (crates/codebase-memory/src/*)
3. HelpOfAi branding layer (helpofai codebase and helpofai graph CLI hooks).

By keeping the engine out of crates/tui UI threads, graph operations do not cause blocking frame drops or Tokio execution hangs.

## Watcher System
The memory graph uses daemon natively. When helpofai starts, codebase-memory manages FS event hooks without bleeding dependencies into helpofai-core.

## Isolation Models
By injecting CBM_CACHE_DIR at runtime, we guarantee the graph instance isolates strictly into %LOCALAPPDATA%\HelpOfAi\engines\codebase-memory rather than polluting generic OS directories.
