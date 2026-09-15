# HelpOfAi System Architecture Schema

## Modules
- `crates/agent`: Orchestration and Constitution enforcement.
- `crates/aios`: Core AIOS runtime abstractions.
- `crates/cli`: CLI interface and command routing.
- `crates/codebase-memory`: Persistent knowledge graph engine (C11 engine + Rust supervisor).
- `crates/config`: Runtime configuration schema.
- `...`

## Data Surfaces
- `%LOCALAPPDATA%/HelpOfAi/engines/codebase-memory`: Engine binaries and UI resources.
- `%LOCALAPPDATA%/HelpOfAi/codebase-memory`: Global graph data, indexes, and runtime logs.
