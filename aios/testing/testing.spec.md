# AIOS Testing Platform — AIOS-MODULE-000010

> **Codename:** Verifier · **Version:** 1.0.0 · **Layer:** 5 (engines)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## 1. Purpose

Orchestrates unit, integration, and contract tests against changed files.
Autodetects test framework and returns structured pass/fail/coverage.

## 2. Contracts

| ID | Capability |
|----|------------|
| `AIOS-CONTRACT-000070` | `unit_testing` — runs suites, reports new failures |

## 3. Dependencies

Requires: kernel, runtime, code. Load order: 21 (optional).