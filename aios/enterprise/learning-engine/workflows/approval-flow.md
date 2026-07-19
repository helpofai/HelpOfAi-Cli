# Learning Engine — Operator Approval Workflow

All framework modifications require operator approval (Engineering Law 16).

## Flow
```
Learning Engine detects pattern
    ↓
Generates proposal: {change_type, description, impact, confidence}
    ↓
Operator receives proposal via CLI
    ↓
Operator: approve / reject / modify
    ↓
If approved → apply change → log in decision journal
If rejected → discard → optionally mark as "seen" to suppress
If modified → apply modified version → log
```

## Proposal Types
- `new_rule`: add a validation rule (e.g., "no hardcoded secrets")
- `workflow_change`: modify a workflow lifecycle phase
- `knowledge_entry`: add to Knowledge Base
- `template_update`: improve a prompt template

## CLI Commands
```
hoa learning proposals           → list pending proposals
hoa learning show <id>           → view proposal detail
hoa learning approve <id>        → approve and apply
hoa learning reject <id>         → reject
hoa learning modify <id>         → open editor to modify
```