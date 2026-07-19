# Knowledge Graph Data Model

Common schema shared by all 9 brain subsystems. Nodes and edges are persisted via the memory module (AIOS-CONTRACT-000060) under namespace `brain_graph`.

## Node Types

| Type | Description | Properties |
|------|-------------|------------|
| `file` | Project file | path, kind, size, content_hash, lang |
| `directory` | Project directory | path, child_count |
| `symbol` | Code symbol | name, kind (class|func|var|interface), file, line |
| `feature` | Feature definition | feature_id, name, status, spec_ref |
| `module` | AIOS module | module_id, version, load_order |
| `package` | External dependency | name, version, ecosystem |
| `decision` | ADR entry | adr_id, title, timestamp |
| `risk` | Risk record | risk_id, severity, status |
| `knowledge` | Learned knowledge | entry_id, topic, confidence |
| `event` | Timeline event | event_type, timestamp, request_id |

## Edge Types

| Type | Description |
|------|-------------|
| `contains` | directory→file, directory→directory |
| `defines` | file→symbol |
| `imports` | file→file, file→package |
| `calls` | symbol→symbol |
| `implements` | file→feature |
| `depends_on` | module→module, feature→feature |
| `decides` | decision→symbol, decision→file |
| `risks` | risk→feature, risk→module |
| `knows` | knowledge→file, knowledge→pattern |
| `affects` | change→file |

## Versioning

The entire graph carries a `graph_version` integer. Each delta from `file_indexing`
increments it. Staleness detection: if `graph_version < last_file_modification`,
brain queries return `BRAIN_STALE` flag.