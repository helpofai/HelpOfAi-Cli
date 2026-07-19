# AIOS Analysis Intelligence — AIOS-MODULE-000007

> **Codename:** Inspector · **Version:** 1.0.0 · **Layer:** 5 (engines)
> **Authority:** Engineering Laws 1, 5, 8, 12 · Constitution Principles 1, 4
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## 1. Purpose

Scores project architecture and security posture. Consults the brain's knowledge
graph for context and produces reports consumed by the planner's risk_analysis
and the review engine.

## 2. Contracts

| ID | Capability |
|----|------------|
| `AIOS-CONTRACT-000050` | `architecture_analysis` — scores cohesion, coupling, layer separation |
| `AIOS-CONTRACT-000051` | `security_analysis` — detects anti-patterns, dependency vulns, dangerous APIs |

## 3. Dependencies

Requires: kernel, runtime, brain. Load order: 10 (optional).