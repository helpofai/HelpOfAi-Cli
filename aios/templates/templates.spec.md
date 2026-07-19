# AIOS Templates — AIOS-MODULE-000021

> **Version:** 1.0.0 · **Layer:** 8 (ecosystem) · **Load order:** 70 (optional)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

Production-grade Markdown templates for all AIOS output types. Each template
uses `{{mustache-style}}` placeholders for engine/variable substitution.

## Available Templates

| Template | Purpose |
|----------|---------|
| `PLAN.md` | Execution plan with steps, gates, cost estimates |
| `EXECUTION_REPORT.md` | Phase-by-phase execution summary |
| `BUG_REPORT.md` | Bug description, reproduction, root cause, fix |
| `CHANGELOG.md` | Version release notes per semantic version |
| `CHECKLIST.md` | Pre/post implementation quality gates |
| `RELEASE_NOTES.md` | Full release notes with known issues |
| `ARCHITECTURE_REPORT.md` | Architecture scoring by dimension |
| `SECURITY_REPORT.md` | Security findings with severity and fix |
| `PERFORMANCE_REPORT.md` | Performance metrics vs budget targets |
| `ROLLBACK_PLAN.md` | Revert procedure for failed phases |
| `ADR_TEMPLATE.md` | Architecture Decision Record format |
| `MODULE_README.md` | Module description with deps and capabilities |
| `CONTRACT_README.md` | Contract interface documentation |
| `WORKFLOW_README.md` | Workflow lifecycle documentation |

## Examples

Workflow execution examples are in `aios/workflows/templates/`:
- `build-feature-output.md` — example feature build result
- `release-output.md` — example release deployment result
- `audit-output.md` — example project audit report

## Usage

Templates are consumed by the Documentation Engine (AIOS-MODULE-000026)
and by individual agents when producing output. The CLI renders templates
by substituting `{{variables}}` with execution context values.