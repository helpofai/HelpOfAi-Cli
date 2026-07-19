# Agent System — Example Workflows

## Example 1: Build Feature
```
Request: hoa build feature "add email auth with JWT"
Dispatch: AIOS-AGENT-000001 (master)
Master spawns:
  → AIOS-AGENT-000002 (architect) — design plan
  → AIOS-AGENT-000003 (backend) — implement auth
  → AIOS-AGENT-000005 (database) — create migration
  → AIOS-AGENT-000007 (qa) — run tests
  → AIOS-AGENT-000016 (reviewer) — review code
Master integrates results → reports to operator
```

## Example 2: Fix Bug
```
Request: hoa fix bug "login returns 500 on expired tokens"
Dispatch: AIOS-AGENT-000001 (master)
Master spawns:
  → AIOS-AGENT-000016 (reviewer) — inspect affected code
  → AIOS-AGENT-000003 (backend) — implement fix
  → AIOS-AGENT-000007 (qa) — verify fix
Master generates rollback → reports to operator
```

## Example 3: Security Audit
```
Request: hoa security audit
Dispatch: AIOS-AGENT-000009 (security) — direct dispatch
Security agent:
  → scans dependencies
  → checks code for vulnerabilities
  → reports findings
```