# AIOS Workflow Engine — AIOS-MODULE-000015

> **Codename:** Factory · **Version:** 1.0.0 · **Layer:** 7 (knowledge) · **Constitution target:** 1.0.0
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## Purpose

Named, repeatable engineering workflows executed as JSON lifecycle phases. Each workflow has triggers, ordered phases, declared engines, and a rollback workflow.

## Defined Workflows (10)

| ID | Name | Triggers | Rollback |
|----|------|----------|----------|
| `AIOS-WORKFLOW-000001` | build-feature | `hoa build feature`, `hoa feature` | WFL-002 |
| `AIOS-WORKFLOW-000002` | rollback | `hoa rollback` | — |
| `AIOS-WORKFLOW-000003` | audit-project | `hoa audit` | — |
| `AIOS-WORKFLOW-000004` | fix-bug | `hoa fix bug`, `hoa debug` | WFL-002 |
| `AIOS-WORKFLOW-000005` | review-code | `hoa review` | — |
| `AIOS-WORKFLOW-000006` | refactor | `hoa refactor` | WFL-002 |
| `AIOS-WORKFLOW-000007` | upgrade | `hoa upgrade` | WFL-002 |
| `AIOS-WORKFLOW-000008` | optimize | `hoa optimize`, `hoa profile` | — |
| `AIOS-WORKFLOW-000009` | analyze | `hoa analyze`, `hoa audit` | — |
| `AIOS-WORKFLOW-000010` | release | `hoa release`, `hoa deploy` | WFL-002 |

## Dependencies

Requires: kernel, runtime, planner, memory. Load order: 50 (optional).