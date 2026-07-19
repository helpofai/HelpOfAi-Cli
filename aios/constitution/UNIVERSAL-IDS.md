# AIOS Universal Identification System v1.0

Every component in AIOS carries a stable, unique, machine-parseable ID.
IDs never change once assigned. They survive module renames, path changes,
and version upgrades.

## ID Format

```
AIOS-{TYPE}-{SERIAL}
```

Examples:
- `AIOS-MODULE-000001` — a loadable subsystem
- `AIOS-ENGINE-000042` — an execution engine
- `AIOS-AGENT-000117` — a specialist agent
- `AIOS-RULE-000523` — an engineering rule
- `AIOS-FEATURE-000089` — a feature definition
- `AIOS-WORKFLOW-000032` — a workflow
- `AIOS-PROMPT-000421` — a prompt template
- `AIOS-TEMPLATE-000078` — a document template
- `AIOS-SCHEMA-000015` — a JSON schema
- `AIOS-CONTRACT-000203` — an interface contract
- `AIOS-SPEC-000451` — a specification
- `AIOS-PACK-000088` — a framework/language/domain pack
- `AIOS-PLUGIN-000009` — a plugin
- `AIOS-SUBSYS-000001` — a subsystem entry in root manifest
- `AIOS-VALIDATOR-000037` — a validation rule
- `AIOS-EVENT-000104` — an event type
- `AIOS-METRIC-000029` — a quality metric

## Serial Numbers

- 6-digit zero-padded serial.
- Unique within type.
- Never reused — deleted IDs leave gaps.

## Mapping to Files

```
AIOS-RULE-000523 → aios/rules/RULE-000523.json
AIOS-SCHEMA-000015 → aios/schemas/SCHEMA-000015.json
AIOS-WORKFLOW-000032 → aios/workflows/WORKFLOW-000032.json + .md
```

## Hashed Content IDs

For extra-large content or external resources:
```
AIOS-HASH-{sha256_first_12}
```

## ID Generation Rules

1. IDs are assigned at file creation time.
2. The module registry maintains the next-available serial per type.
3. Manual assignment is preferred: author picks the next available number.
4. After assignment, the ID is immutable — even if the file moves or is renamed.

## Cross-Referencing

```
{
  "rule_refs": ["AIOS-RULE-000523", "AIOS-RULE-000091"],
  "depends_on": ["AIOS-ENGINE-000042"],
  "provides": ["AIOS-CAPABILITY-000218"]
}
```

Cross-references use these IDs exclusively — never relative paths.

## Runtime Resolution

The CLI resolves `AIOS-{NAME}-XXXXXX}` to:
1. The canonical file path in the `/aios` directory tree
2. The latest version of the referenced artifact
3. All transitive dependencies declared in the artifact's manifest

Resolution never relies on directory layout guessing.