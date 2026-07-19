# Bug Engine — AIOS-MODULE-000024

> **Codename:** Debugger · **Version:** 1.0.0 · **Load order:** 25 (optional)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

Automated bug detection and auto-fix engine. Given error logs, stack traces, or test failures, identifies root causes and suggests or applies fixes with rollback plans.

**Contracts:**
- `AIOS-CONTRACT-000102` — bug_detection: root cause analysis from evidence
- `AIOS-CONTRACT-000103` — auto_fix: diff+rollback fix generation