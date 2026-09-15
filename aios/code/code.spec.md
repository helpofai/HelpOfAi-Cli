# AIOS Code Engine — AIOS-MODULE-000009

> **Codename:** Forger · **Version:** 1.0.0 · **Layer:** 5 (engines)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## 1. Purpose

Generates production code from approved plans and feature specs. Every output
carries a traceable origin. Respects existing conventions — never reformats
unrelated code.

## 2. Contracts

| ID | Capability |
|----|------------|
| `AIOS-CONTRACT-000060` | `code_generation` — create/edit/delete files with trace_back |

## 3. Dependencies

Requires: kernel, planner. Optional: brain, feature. Load order: 20 (optional).
### 4. Codebase Memory Integration
Before modifying large blocks of code, use targeted graph queries against the codebase-memory MCP engine (if available).
Do not allow the agent to blindly edit files without understanding relevant relationships when graph information is accessible locally via search_graph or 	race_path.
