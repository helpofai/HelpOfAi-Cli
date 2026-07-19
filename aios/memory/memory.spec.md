# AIOS Engineering Memory — AIOS-MODULE-000006

> **Codename:** Vault · **Version:** 1.0.0 · **Layer:** 4 (intelligence) · **Constitution target:** 1.0.0
> **Authority:** Engineering Laws 5, 9, 10, 13, 14 · Constitution Principles 6, 14
> **Machine contract:** [`aios/memory/module.json`](./module.json) (authoritative)

## 1. Purpose

The Engineering Memory is the persistence layer. Every module that needs durable
state — the kernel's state machine history, the runtime's event log and cache,
the planner's plans, the brain's knowledge graph — writes through this module.
Nothing that must survive a process restart lives outside of memory.

## 2. Architecture

- **Pattern:** Clean Architecture — a single `persistence` contract with
  namespace-switched backends: `state`, `events`, `plans`, `brain_graph`,
  `cache`, `logs`.
- **Loading:** `load_order: 5`. Required: planner and brain depend on it.
- **Depends on:** kernel (`000002`), runtime (`000003`).
- **Budget:** load ≤ 100 ms, memory ≤ 128 MB, cache ≤ 512 MB.

## 3. Contracts

| ID | Capability | Contract file |
|----|------------|---------------|
| `AIOS-CONTRACT-000060` | `persistence` | [`contracts/persistence.json`](./contracts/persistence.json) |

## 4. Storage Model

- **`state`**: per-request state machine transitions (keyed by `AIOS-REQ-NNNNNN`).
- **`events`**: append-only event log, monotonic sequence per namespace.
- **`plans`**: approved execution plans (keyed by `AIOS-PLAN-NNNNNN`).
- **`brain_graph`**: serialized knowledge graph, versioned.
- **`cache`**: runtime cache backing store (LRU pruned).
- **`logs`**: structured JSON-lines logs, rotated at `max_size_mb`.

All files live under `aios/.cache/<namespace>/`.

## 5. Guarantees

- **Atomic per-op**: put/append/delete complete or fail — no partial writes.
- **Monotonic sequence**: every `append` returns a strictly increasing sequence
  number for the namespace, guaranteeing order for audit (Law 13 traceability).
- **Namespace isolation**: corruption in one namespace (e.g. `cache`) does not
  affect another (e.g. `events`).

## 6. Events

- `AIOS-EVENT-000030` `memory_corrupt` (emitted when a stored value fails schema
  re-validation on load).

## 7. Failure

- `MEMORY_NS_FULL` → reject, surface namespace + quota.
- `MEMORY_NS_CORRUPT` → quarantine file, return NotFound, emit `memory_corrupt`.
- `MEMORY_IO_FAILED` → fail the op; callers degrade (Law 9).

**Plugin failures cannot corrupt memory** (Law 16): only trusted core modules
(`kernel`, `runtime`, `planner`, `brain`) may write; plugins read through
contracts.

## 8. Self-Test

Per Engineering Law 11:

- Persistence contract JSON parses against `AIOS-SCHEMA-000015`.
- Atomicity: interrupted write → file unchanged or complete, never partial.
- Monotonicity: 10 sequential appends → sequences are contiguous [n, n+9].
- Namespace isolation: corrupt `cache` entry → `events` namespace still readable.
- Quota enforcement: put after quota exceeded → MEMORY_NS_FULL.

Test entry: `aios/memory/tests/memory-selftest.json`.