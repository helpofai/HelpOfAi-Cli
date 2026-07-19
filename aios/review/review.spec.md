# AIOS Review Engine — AIOS-MODULE-000011

> **Codename:** Critic · **Version:** 1.0.0 · **Layer:** 5 (engines)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## 1. Purpose

Senior-engineer-style automated review. Analyzes code changes against conventions,
architecture rules, and best practices. Returns actionable comments by severity.

## 2. Contracts

| ID | Capability |
|----|------------|
| `AIOS-CONTRACT-000080` | `code_review` |

## 3. Dependencies

Requires: kernel, brain. Load order: 22 (optional).