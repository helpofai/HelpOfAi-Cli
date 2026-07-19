# AIOS Project Brain — AIOS-MODULE-000005

> **Codename:** Memory · **Version:** 1.0.0 · **Layer:** 4 (intelligence) · **Constitution target:** 1.0.0
> **Authority:** Engineering Laws 8 (Cache Aggressively), 14 (Knowledge Persistence)
> **Machine contract:** [`aios/brain/module.json`](./module.json) (authoritative)

## 1. Purpose

The Project Brain builds and maintains the project knowledge graph — a persisted,
incremental representation of the workspace's files, symbols, dependencies,
routes, and architecture decisions. It survives across sessions (Law 14) and
short-circuits re-analysis of unchanged files through hashed invalidation (Law 8).

## 2. Architecture

- **Pattern:** Clean Architecture + Event-Driven (emits `graph_updated` on index).
- **Loading:** `load_order: 4`. Required.
- **Depends on:** kernel (`000002`), memory (`000006`).
- **Budget:** load ≤ 500 ms, memory ≤ 512 MB, cache ≤ 256 MB.

Two contracts:

```
   project_root / changed_files[]
        │
        ▼
   file_indexing ─▶ graph_delta (persisted to memory)
        │
        ▼
   project_understanding ─▶ query → answer (graph slice)
```

## 3. Contracts

| ID | Capability | Contract file |
|----|------------|---------------|
| `AIOS-CONTRACT-000040` | `project_understanding` | [`contracts/project-understanding.json`](./contracts/project-understanding.json) |
| `AIOS-CONTRACT-000041` | `file_indexing`         | [`contracts/file-indexing.json`](./contracts/file-indexing.json) |

## 4. Indexing Model

`file_indexing` is **incremental**: only files whose hash changed since the last
index are re-parsed. This satisfies:
- Law 8 (Cache Aggressively): unchanged files = unchanged graph nodes.
- Law 14 (Knowledge Persistence): the graph survives sessions; on restart,
  only new/changed files are re-indexed.

The graph stores nodes (file, symbol, decision, dependency) and edges (calls,
imports, provides, conflicts, annotates). The persisted format is versioned
(`graph_version` incremented per delta).

## 5. Query Model

`project_understanding` returns graph slices, never raw file content:
- "what calls X" → edges of kind `calls` terminating at symbol X.
- "what depends on Y" → edges of kind `depends_on` originating from module Y.
- "what changed since Z" → nodes/edges with `updated_at > Z`.

Staleness is flagged when the graph version is older than the latest file change.

## 6. Events

- `AIOS-EVENT-000040` `graph_updated` (emitted after `file_indexing` completes).

## 7. Failure

- `BRAIN_NOT_INDEXED` → no graph; run `file_indexing` first.
- `BRAIN_STALE` → flag staleness, return best slice with marker.
- `BRAIN_PARSE_FAILED` → record file as `unparsed`, continue, log missing language pack.
- `BRAIN_WRITE_FAILED` → keep in-memory graph, degrade (Law 9).

## 4. Brain Subsystems

Nine subsystems, each with a dual-format spec (`.json` metadata + `.md` human docs):

| ID | Brain | Cache tier | Purpose |
|----|-------|-----------|---------|
| `AIOS-BRAIN-000001` | Project Brain | file-level | File index, directory structure, language/framework detection |
| `AIOS-BRAIN-000002` | Feature Brain | on-demand | Feature-to-file mapping, feature coverage |
| `AIOS-BRAIN-000003` | Dependency Brain | dependency-level | Module/package/symbol dependency graph, cycle detection |
| `AIOS-BRAIN-000004` | Context Brain | query-level | Context assembly for requests — selects relevant brain slices |
| `AIOS-BRAIN-000005` | Decision Brain | append-only | ADR system — every decision recorded for traceability |
| `AIOS-BRAIN-000006` | Knowledge Brain | append-only | Learned knowledge — project conventions, patterns |
| `AIOS-BRAIN-000007` | Execution Brain | execution-level | Execution traces for resumability and audit |
| `AIOS-BRAIN-000008` | History Brain | append-only | Engineering timeline — append-only event log |
| `AIOS-BRAIN-000009` | Risk Brain | risk-level | Risk tracking, scoring, and mitigation |

All brains share the **Knowledge Graph Data Model** (`[models/knowledge-graph.md](./models/knowledge-graph.md)`)
and **Cache Strategy** (`[rules/cache-strategy.md](./rules/cache-strategy.md)`)

## 5. Knowledge Graph

A shared graph with 10 node types and 10 edge types (see
[`models/knowledge-graph.md`](./models/knowledge-graph.md)). The graph is versioned:
each `file_indexing` delta increments `graph_version`. Staleness detection
compares `graph_version` against the latest file modification time.

## 6. Cache & Incremental Updates

See [`rules/cache-strategy.md`](./rules/cache-strategy.md). Key rules:
- File-level updates are fast and frequent.
- Dependency-level updates are slow and triggered only on manifest changes.
- Decision/Risk/Knowledge are append-only — never rebuilt unless forced.
- LRU eviction at `max_cache_mb = 256 MB`.

## 7. Brain Interactions

```
request_text
    │
    ▼
Context Brain ─▶ selects relevant slices from other brains
    │
    ├── Project Brain ──▶ file list matching request
    ├── Feature Brain ──▶ relevant feature specs
    ├── Dependency Brain ──▶ affected modules
    ├── Knowledge Brain ──▶ relevant conventions
    └── Risk Brain ──▶ risk profile for scope
        │
        ▼
    context_pack → kernel → planner
```

## 8. Events

- `AIOS-EVENT-000040` `graph_updated` (after file_indexing completes)
- `AIOS-EVENT-000041` `brain_query` (per query)
- `AIOS-EVENT-000042` `decision_recorded` (per ADR)
- `AIOS-EVENT-000043` `risk_assessed` (per risk update)
- `AIOS-EVENT-000044` `knowledge_learned` (per knowledge entry)

## 9. Self-Test

Per Engineering Law 11:

- Both contract JSONs parse against `AIOS-SCHEMA-000015`.
- All 9 brain subsystem JSONs parse against `AIOS-SCHEMA-000015`.
- Deterministic fixture: same project_root → same `graph_version`.
- Incremental fixture: only `changed_files` produce delta; unchanged files skip.
- Staleness detection: graph_version older than touched files → `BRAIN_STALE` flag.
- Context pack assembly: given a request, returns a relevant slice (not the full graph).

Test entry: `aios/brain/tests/brain-selftest.json`.