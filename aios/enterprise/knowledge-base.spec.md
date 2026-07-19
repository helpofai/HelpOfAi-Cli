# Knowledge Base — AIOS-ENTERPRISE-000070

Searchable repository of project knowledge. Built on the Knowledge Brain
(AIOS-BRAIN-000006). Queried by agents during context assembly for
context-aware generation.

## Content Types
- **Conventions**: team coding standards, naming, patterns
- **Known issues**: recurring problems and their solutions
- **Architecture decisions**: ADRs (synced from Decision Brain)
- **Domain knowledge**: business rules, glossary, domain concepts
- **Team practices**: branching strategy, review process, deploy cadence

## Search
Full-text search with relevance scoring. Agents receive the top-k results
during context assembly via the Context Brain.

## CLI Commands
```
hoa knowledge search <query>    → search knowledge base
hoa knowledge add <title>       → add entry interactively
hoa knowledge export <id>       → export entry
```