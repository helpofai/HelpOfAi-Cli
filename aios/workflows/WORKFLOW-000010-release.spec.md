# Release — AIOS-WORKFLOW-000010

**Triggers:** `hoa release`, `hoa deploy`
**Gate:** tests_pass, review_approved
**Rollback:** AIOS-WORKFLOW-000002
**Est. duration:** 10-45 min

## Lifecycle
1. **Validate** — Testing Platform — gate: tests_pass
2. **Review** — Review Engine — gate: review_approved
3. **Document** — Documentation Engine generates changelog
4. **Deploy** — DevOps Platform orchestrates deployment
5. **Release Notes** — Documentation Engine writes release notes