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