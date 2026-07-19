# AIOS Metadata Standards v1.0

Every AIOS artifact that ships human documentation (.md) must also ship
machine-readable metadata (.json) so the CLI can consume it without NLP.

## Dual-Format Rule

For every specification, workflow, agent, engine, rule, or template
that ships in AIOS, both formats are required:

```
workflows/WORKFLOW-000001-build-feature.spec.md   ← Human docs
workflows/WORKFLOW-000001-build-feature.json      ← Machine metadata

agents/AGENT-000042-android-agent.md             ← Human docs
agents/AGENT-000042-android-agent.json           ← Machine metadata
```

The `.json` file is authoritative for the CLI.

## Metadata Structure

Every metadata file follows one of the registered schemas:

| Artifact type  | Schema ID          |
|---------------|---------------------|
| Module         | SCHEMA-000011       |
| Dependency     | SCHEMA-000012       |
| Workflow       | SCHEMA-000013       |
| Agent          | SCHEMA-000014       |
| Contract       | SCHEMA-000015       |
| Feature        | SCHEMA-000020       |
| Prompt         | SCHEMA-000021       |
| Template       | SCHEMA-000022       |
| Rule           | SCHEMA-000023       |
| Report         | SCHEMA-000024       |

## Consistency Contract

The following properties must be identical across both formats:

- `id`
- `name`
- `version`
- `depends_on`
- `provides`

The CLI will reject a module if the two formats disagree on any of these.

## CLI Consumption

```
hoa load AIOS-WORKFLOW-000032
  → reads only WORKFLOW-000032-*.json
  → validates against AIOS-SCHEMA-000013
  → resolves dependencies
  → renders human doc only when user requests --explain
```

## Authoring Convention

1. Write the `.json` metadata first (it's the machine contract).
2. Derive the `.md` spec from the metadata.
3. Keep them in sync — CI gates enforce format consistency.

## Metadata Envelope

Every JSON metadata file starts with:

```json
{
  "$schema": "./schemas/SCHEMA-000011-module.json",
  "id": "AIOS-...",
  "name": "...",
  "version": "..."
}
```

The `$schema` field tells the CLI which validator to run.