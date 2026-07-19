# AIOS Feature Intelligence — AIOS-MODULE-000008

> **Codename:** Architect · **Version:** 1.0.0 · **Layer:** 5 (engines)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## 1. Purpose

Produces structured feature specs with boundaries, interfaces, data models,
acceptance criteria, and test scope. Analyzes feature dependencies against
existing features.

## 2. Contracts

| ID | Capability |
|----|------------|
| `AIOS-CONTRACT-000082` | `feature_specification` |
| `AIOS-CONTRACT-000083` | `feature_dependency_analysis` |

## 3. Dependencies

Requires: planner, brain. Load order: 11 (optional).