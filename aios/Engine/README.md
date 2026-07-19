# AIOS Engine — Control Center Hub

> **Version:** 1.0.0 · **Module ID:** AIOS-MODULE-000022
> **Role:** System dashboard and orchestration hub

The Engine module is the top-level control center for AIOS. It provides:

- **System dashboard** — real-time status of all 28 modules, brain health, active workflows
- **Command hub** — unified entry point for all AIOS CLI commands
- **Orchestration** — verifies all subsystems are operational and aligned
- **Performance monitoring** — aggregates health checks from all modules

## Relationships

```
Engine (AIOS-MODULE-000022) — AIOS Control Center
├── reads: Registry, Brain, Kernel status
├── monitors: all module health checks
└── provides: unified CLI entry point ("hoa" commands)
```

## Health Dashboard

```
hoa status
→ AIOS v1.0.0 — All systems nominal
  Modules: 28/28 loaded
  Brain: indexed (5 min ago)
  Cache: 89% hit rate
  Workflows: 12 completed, 0 failed today
```