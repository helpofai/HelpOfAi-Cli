# AIOS Kernel — AIOS-MODULE-000002

> **Codename:** Foundation · **Version:** 1.0.0 · **Layer:** 2 (kernel) · **Constitution target:** 1.0.0
> **Authority:** Constitution Principles 1, 4, 6, 14 · Engineering Laws 1, 5, 9, 13, 16
> **Machine contract:** [`aios/kernel/module.json`](./module.json) (authoritative)

## 1. Purpose

The Kernel is the read-only core that turns a natural-language request into a
deterministic, traceable execution. It does **not** generate engineering output
itself — it routes. Per Constitution Principle 1, the kernel embodies the rule
**"90% routing, 10% generation."** All generation is delegated to capability
providers loaded on demand.

The Kernel is **immutable to plugins** (Engineering Law 16). Extensions interact
with it only through the four public contracts listed in §3.

## 2. Architecture

- **Pattern:** Clean Architecture + Event-Driven transitions.
- **Loading:** `load_order: 1`. Required: AIOS cannot boot without it.
- **Depends on:** `AIOS-ROOT-000000`, `AIOS-CONST-000001` (constitution).
- **Permissions:** filesystem `aios/` (trusted), process (trusted).
- **Budget:** load ≤ 200 ms, memory ≤ 64 MB, cache ≤ 32 MB.

The Kernel is composed of four contracts, each a pure function over its inputs
except for declared side effects (events emitted, state persisted):

```
request_text ─▶ request_routing ─▶ CapabilityDemand
                                          │
                                          ▼
                                 capability_routing ─▶ Resolution
                                          │
                                          ▼
                                  execution_lifecycle ─▶ ExecutionReport
                                          │
                                          ▼
                                    state_machine (per phase, per request)
```

## 3. Contracts

| ID | Capability | Contract file |
|----|------------|---------------|
| `AIOS-CONTRACT-000010` | `request_routing`   | [`contracts/request-routing.json`](./contracts/request-routing.json) |
| `AIOS-CONTRACT-000011` | `execution_lifecycle` | [`contracts/execution-lifecycle.json`](./contracts/execution-lifecycle.json) |
| `AIOS-CONTRACT-000012` | `capability_routing` | [`contracts/capability-routing.json`](./contracts/capability-routing.json) |
| `AIOS-CONTRACT-000013` | `state_machine`     | [`contracts/state-machine.json`](./contracts/state-machine.json) |

All four are validated against `AIOS-SCHEMA-000015` (contract schema).

## 4. Execution Lifecycle

Canonical phase order, each a gated checkpoint:

1. **Understand** — parse intent from the request.
2. **Discover** — consult the brain/memory for relevant project context.
3. **Analyze** — score architecture, surface risks (analysis module).
4. **Research** — pull framework/language/domain pack knowledge.
5. **Simulation** — dry-run the plan against the digital twin (optional).
6. **Planning** — planner emits the execution plan (gate: plan_approved).
7. **Implementation** — code engine produces changes (gate: build_passes).
8. **Validation** — schema/contract conformance (gate: contracts_met).
9. **Testing** — unit/integration tests run (gate: tests_pass).
10. **Review** — review engine + reviewer agent (gate: review_approved).
11. **Documentation** — docs updated automatically (Law 6).
12. **Completion** — execution_report emitted, traces persisted.

A failed gate on a non-skippable phase triggers the configured
`rollback_workflow` and holds the run at that phase (Law 4, Law 9).

## 5. State Machine

States: `idle` → `understanding` → `discovering` → `analyzing` → `researching`
→ `simulating` → `planning` → `implementing` → `validating` → `testing` →
`reviewing` → `documenting` → `completed` | `failed` | `paused`.

Transitions are deterministic (same `from` + `event_type` → same `to`).
Every transition is appended to a per-request audit trail persisted to
`AIOS-MODULE-000006` (memory) so Engineering Law 13 (Complete Traceability)
holds across sessions.

## 6. Events Emitted

- `AIOS-EVENT-000001` `request_received`
- `AIOS-EVENT-000002` `phase_started`
- `AIOS-EVENT-000003` `phase_completed`
- `AIOS-EVENT-000004` `state_changed`

## 7. Failure & Recovery

- Empty request → `KERNEL_EMPTY_REQUEST` → reprompt.
- Unroutable intent → `KERNEL_UNROUTABLE` → list candidate packs (Law 12).
- Ambiguous routing (confidence gap < 0.15) → `KERNEL_AMBIGUOUS` → ranked list.
- Phase gate failed → `KERNEL_PHASE_GATE_FAILED` → rollback + hold.
- Planner missing at Planning → `KERNEL_PLANNER_MISSING` → pause or external plan.
- Illegal transition → `KERNEL_ILLEGAL_TRANSITION` → reject, keep state.
- Memory persist failed → `KERNEL_PERSIST_FAILED` → degrade, warn operator (Law 9).

## 8. Traceability

Every execution emits an `ExecutionReport` linking each artifact back through:

```
Feature → Workflow → Phase → Engine → Contract → Output
```

This satisfies Constitution Principle 14 (Trace Everything) — no AIOS output
reaches the operator without a chain back to its producing contract.

## 9. Immutability

Per Engineering Law 16, no plugin may modify the kernel. New runtime behavior is
added by declaring a new capability in another module and routing to it — never
by patching kernel contracts. Public contract IDs (`AIOS-CONTRACT-000010` …
`-000013`) are backward-compatible forever per Constitution Principle 9.

## 10. Self-Test

Per Engineering Law 11, the kernel ships with a contract self-test that
validates, offline and with no model calls:

- Each contract JSON parses against `AIOS-SCHEMA-000015`.
- Every `provides` capability in `module.json` has a matching contract file.
- Every contract `id` matches its `name` against `capabilities.json`.
- The state machine transition graph is closed (no dangling `to` states).
- The lifecycle phase list matches the state machine state list.

Test entry: `aios/kernel/tests/kernel-selftest.json` (run by `hoa validate
AIOS-MODULE-000002`).
