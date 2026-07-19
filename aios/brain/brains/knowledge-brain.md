# Knowledge Brain — AIOS-BRAIN-000006

Learns and stores knowledge across sessions. Unlike the static knowledge packs
(frameworks, languages, domains), this brain accumulates project-specific and
team-specific knowledge.

### Sources
- Code patterns detected by analysis engine
- Conventions observed during code generation
- Operator feedback and corrections
- Explicit knowledge entries from the decision journal

### Schema
- `entries[]`: id, topic, content, source (detected|feedback|explicit)
- `confidence`: 0.0-1.0, decays over time unless reinforced
- `applies_to[]`: file paths or patterns the knowledge applies to