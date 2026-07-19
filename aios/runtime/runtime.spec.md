# AIOS Runtime — AIOS-MODULE-000003

> **Codename:** Heartbeat · **Version:** 1.0.0 · **Layer:** 3 (runtime) · **Constitution target:** 1.0.0
> **Authority:** Constitution Principles 1, 4, 6, 14 · Engineering Laws 4, 8, 9, 10, 14, 15, 16
> **Machine contract:** [`aios/runtime/module.json`](./module.json) (authoritative)

## 1. Purpose

The Runtime is the execution substrate of AIOS. It owns the durable job queue,
the deterministic priority scheduler, the parallel-execution coordinator,
the incremental content-addressed cache, the inter-module event bus, and the
structured logging pipeline. Long-lived state and cross-module events
**always** flow through this module, never bypass it.

The Runtime is **read-only core** (Constitution Principle 6) and **immutable to
plugins** (Engineering Law 16). Extensions gain behavior by subscribing to the
event bus and declaring new capabilities — never by patching runtime contracts.

## 2. Architecture

- **Pattern:** Clean Architecture + Event-Driven.
- **Loading:** `load_order: 2`. Required: AIOS cannot execute without it.
- **Depends on:** `AIOS-MODULE-000002` (kernel).
- **Permissions:** filesystem `aios/.cache/` (trusted), process (trusted), model (trusted).
- **Budget:** load ≤ 100 ms, memory ≤ 256 MB, cache ≤ 512 MB.

```
                        ┌─── job_queue (durable FIFO + priority)
   CapabilityDemand ───▶│
                        ├─── scheduler (deterministic dispatch)
                        │        │
                        │        ▼
                        ├─── parallel_execution (isolated contexts)
                        │        │
                        │        ▼
                        ├─── cache_manager (content-addressed, incremental)
                        │
                        ├─── event_bus (pub/sub, append-only log)
                        │
                        └─── logging (JSON-lines, trace-id correlated)
```

## 3. Contracts

| ID | Capability | Contract file |
|----|------------|---------------|
| `AIOS-CONTRACT-000020` | `job_queue`          | [`contracts/job-queue.json`](./contracts/job-queue.json) |
| `AIOS-CONTRACT-000021` | `scheduler`          | [`contracts/scheduler.json`](./contracts/scheduler.json) |
| `AIOS-CONTRACT-000022` | `parallel_execution` | [`contracts/parallel-execution.json`](./contracts/parallel-execution.json) |
| `AIOS-CONTRACT-000023` | `cache_manager`      | [`contracts/cache-manager.json`](./contracts/cache-manager.json) |
| `AIOS-CONTRACT-000024` | `event_bus`          | [`contracts/event-bus.json`](./contracts/event-bus.json) |
| `AIOS-CONTRACT-000025` | `logging`            | [`contracts/logging.json`](./contracts/logging.json) |

All six validate against `AIOS-SCHEMA-000015`.

## 4. Job Lifecycle

1. Kernel routes a `CapabilityDemand` → engine.
2. Runtime **enqueues** a job (`AIOS-CONTRACT-000020`) tagged with priority (1–10),
   target engine, and a trace_id.
3. **Scheduler** (`AIOS-CONTRACT-000021`) drains the queue deterministically:
   same queue snapshot → same dispatch order (Constitution Principle 4).
4. **Parallel execution** (`AIOS-CONTRACT-000022`) runs isolated sub-agent / engine
   contexts, merges results with a deterministic `merge_key`.
5. The **cache** (`AIOS-CONTRACT-000023`) short-circuits unchanged work; an entry
   is valid only if every declared `input_hashes` value still matches.
6. Every transition is published on the **event bus** (`AIOS-CONTRACT-000024`)
   and recorded in the **log** (`AIOS-CONTRACT-000025`) with the same `trace_id`.

## 5. Determinism Guarantee

For a fixed set of inputs — queue contents, scheduler tick, cache state, and
registered subscribers — the runtime produces the **same** dispatched set, the
**same** `merge_key`, and the **same** log sequence modulo timestamps. Any
non-determinism (wall-clock, model temperature) is explicitly marked in the
originating engine's contract, never introduced silently by the runtime.

## 6. Events Emitted

- `AIOS-EVENT-000010` `job_enqueued` · `AIOS-EVENT-000011` `job_dequeued`
- `AIOS-EVENT-000012` `job_dispatched` · `AIOS-EVENT-000013` `job_deferred`
- `AIOS-EVENT-000014` `task_started` · `AIOS-EVENT-000015` `task_completed`
- `AIOS-EVENT-000016` `task_timeout` · `AIOS-EVENT-000017` `cache_corrupt`

## 7. Caching & Hygiene

- Cache lives under `aios/.cache/<producer_module>/<key>`.
- LRU pruning kicks in at `performance_budget.max_cache_mb` (Law 10).
- Isolated parallel contexts are torn down after completion (Law 15).
- Logs rotate when the active file exceeds the configured `max_size_mb`.

## 8. Failure & Recovery (Graceful — Law 9)

- Queue full → `RUNTIME_Q_FULL` → reject enqueue, advise drain.
- Job missing → `RUNTIME_JOB_NOT_FOUND` → return NotFound, no blind retry.
- Engine over budget → `RUNTIME_ENGINE_OVER_BUDGET` → defer until in-flight drains.
- Task timeout → `RUNTIME_TASK_TIMEOUT` → cancel, keep partial output, emit `task_timeout`.
- Deadlock → `RUNTIME_DEADLOCK` → abort batch, surface cycle (never silently retry).
- Cache corrupt → `RUNTIME_CACHE_CORRUPT` → drop entry, return miss, never return bad data.
- Handler failed → `RUNTIME_HANDLER_FAILED` → log + degrade subscriber, continue delivery.

**Plugin/extension failures never crash the runtime** (Law 16): a failing
subscriber is marked degraded and skipped; the next subscriber still gets the
event.

## 9. Traceability

Every dispatched job carries a `trace_id` (`AIOS-REQ-NNNNNN`). Logs are written
as JSON lines under `aios/.cache/logs/aios-<date>.jsonl` with that `trace_id`,
satisfying Engineering Law 13 and Constitution Principle 14. The event log is
append-only and is the audit surface for any post-hoc review.

## 10. Immutability

Per Engineering Law 16, plugins may **subscribe** to the event bus and
**declare** capabilities but may not override scheduler ordering, queue
semantics, or cache invalidation rules. New runtime behavior must come from a
*new* contract in another module, never from a patch to these six.

## 11. Self-Test

Per Engineering Law 11, the runtime ships with an offline self-test that
validates:

- Each of the 6 contract JSON files parses against `AIOS-SCHEMA-000015`.
- The `provides` list in `module.json` exactly matches the 6 contract files.
- Every `AIOS-CONTRACT-RRRRRR` referenced by the event_bus / scheduler example
  payloads resolves to a present contract.
- Scheduler determinism fixture: same queue snapshot → same dispatch order.

Test entry: `aios/runtime/tests/runtime-selftest.json` (run by
`hoa validate AIOS-MODULE-000003`).
