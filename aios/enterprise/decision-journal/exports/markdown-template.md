# Decision Journal — Markdown Export Template

```markdown
# Decision Journal Export

Generated: {{export_date}}
Filter: {{filter_criteria}}

---

## [{{id}}] {{title}}
- **Status:** {{status}}
- **Tags:** {{tags}}
- **Date:** {{date}}

### Context
{{context}}

### Decision
{{decision}}

### Consequences
{{consequences}}

---
```

Usage: `hoa decision export --all --format markdown > all-adrs.md`