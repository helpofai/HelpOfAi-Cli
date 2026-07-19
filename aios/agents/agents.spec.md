# AIOS Agent System — AIOS-MODULE-000014

> **Codename:** Orchestrator · **Version:** 1.0.0 · **Layer:** 6 (agents) · **Constitution target:** 1.0.0
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## Purpose

Multi-agent orchestration system. The Master Agent decomposes goals, dispatches to specialist agents, and synthesizes results.

## Defined Agents (16)

| ID | Name | Role | Domain |
|----|------|------|--------|
| `AIOS-AGENT-000001` | Master Agent | master | engineering — orchestrates all specialists |
| `AIOS-AGENT-000002` | Architect Agent | architect | architecture — design & review |
| `AIOS-AGENT-000003` | Backend Agent | backend | backend — API & services |
| `AIOS-AGENT-000004` | Frontend Agent | frontend | frontend — UI components |
| `AIOS-AGENT-000005` | Database Agent | database | data — schemas & migrations |
| `AIOS-AGENT-000006` | API Agent | api | backend — API design & contracts |
| `AIOS-AGENT-000007` | QA Agent | qa | testing — test strategy & cases |
| `AIOS-AGENT-000008` | DevOps Agent | devops | devops — CI/CD & deployment |
| `AIOS-AGENT-000009` | Security Agent | security | security — audits & vulns |
| `AIOS-AGENT-000010` | Documentation Agent | documentation | docs — READMEs & API refs |
| `AIOS-AGENT-000011` | Android Agent | backend | mobile — Kotlin/Compose |
| `AIOS-AGENT-000012` | iOS Agent | frontend | mobile — Swift/SwiftUI |
| `AIOS-AGENT-000013` | Flutter Agent | flutter | mobile — Dart/Flutter |
| `AIOS-AGENT-000014` | Laravel Agent | backend | backend — PHP/Laravel |
| `AIOS-AGENT-000015` | React Agent | frontend | frontend — React/Next.js |
| `AIOS-AGENT-000016` | Reviewer Agent | reviewer | engineering — senior code review |

## Dependencies

Requires: kernel, runtime, planner. Load order: 30 (optional).