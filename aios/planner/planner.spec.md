# AIOS Planner — AIOS-MODULE-000004

> **Codename:** Strategist · **Version:** 1.0.0 · **Layer:** 4 (intelligence) · **Constitution target:** 1.0.0
> **Authority:** Constitution Principles 1, 4, 12, 13 · Engineering Laws 1, 4, 5, 12, 13
> **Machine contract:** [`aios/planner/module.json`](./module.json) (authoritative)

## 1. Purpose

The Planner decomposes a high-level `CapabilityDemand` into an ordered, gated
`ExecutionPlan`. Every step is tagged with the capability that serves it, a
gate that must pass, and a rollback reference (Law 4). The kernel's
`execution_lifecycle` pauses at the Planning phase until this module emits an
approved plan.

The Planner embodies Engineering Law 12: when a goal is too vague or requires a
capability that isn't installed, it **never** fabricates steps — it surfaces
what's missing and requests clarification.

## 2. Architecture

- **Pattern:** Clean Architecture + Hexagonal (isolated from execution details).
- **Loading:** `load_order: 3`. Required: AIOS cannot plan without it.
- **Depends on:** kernel (`000002`), runtime (`000003`). Optional: brain (`000005`).
- **Budget:** load ≤ 300 ms, memory ≤ 128 MB, cache ≤ 64 MB.

The three contracts compose linearly:

```
   CapabilityDemand
        │
        ▼
   goal_decomposition ─▶ ExecutionPlan
        │
        ├─▶ risk_analysis ─▶ RiskReport (per step)
        │
        └─▶ cost_estimation ─▶ CostReport (budget checks)
                             │
                             ▼
                        plan_approved | plan_rejected
```

## 3. Contracts

| ID | Capability | Contract file |
|----|------------|---------------|
| `AIOS-CONTRACT-000030` | `goal_decomposition` | [`contracts/goal-decomposition.json`](./contracts/goal-decomposition.json) |
| `AIOS-CONTRACT-000031` | `risk_analysis`     | [`contracts/risk-analysis.json`](./contracts/risk-analysis.json) |
| `AIOS-CONTRACT-000032` | `cost_estimation`   | [`contracts/cost-estimation.json`](./contracts/cost-estimation.json) |

All three validate against `AIOS-SCHEMA-000015`.

## 4. Plan Lifecycle

1. **goal_decomposition**: demand → tree of `steps[]`, each with `capability_id`,
   `gate`, `depends_on[]`, `rollback_ref`.
2. **risk_analysis**: scores each step on likelihood/impact/reversibility/blast_radius,
   assigning a `risk_class` (low → critical). Irreversible high-impact steps are
   `blocking_steps[]` requiring operator approval (Law 4).
3. **cost_estimation**: estimates tokens, minutes, disk delta, files touched, and
   checks each step against the target engine's `performance_budget`. Violations
   are surfaced in `budget_violations[]`.

The plan is **not auto-approved** if any of these hold: a `blocking_steps[]`
entry, a `budget_violations[]` entry, or `confidence < 0.6` from the kernel's
`request_routing`. The operator receives the plan for explicit gate before the
lifecycle transitions to Implementation.

## 5. Determinism

Same `CapabilityDemand` + same installed capabilities + same brain snapshot →
same `plan_id`, same `steps[]` ordering, same `risk_class` per step
(Constitution Principle 4). Cost estimates may vary with dynamic engine budgets;
that variance is declared in the `cost_report.notes`.

## 6. Events

- `AIOS-EVENT-000020` `plan_drafted`
- `AIOS-EVENT-000021` `plan_approved` | `plan_rejected`
- `AIOS-EVENT-000022` `risk_assessed`
- `AIOS-EVENT-000023` `cost_estimated`

## 7. Failure

- `PLANNER_GOAL_TOO_VAGUE` → request clarification (Law 12).
- `PLANNER_CAPABILITY_GAP` → mark step blocked, plan the resolvable rest.
- `PLANNER_CYCLE` → break minimum-feedback-edge, log which was cut.
- `PLANNER_BUDGET_VIOLATION` → surface violation, suggest split step.
- `PLANNER_RISK_REVERSIBILITY_ZERO` → require operator gate + rollback strategy.
- `PLANNER_NO_ENGINE_FOR_STEP` → mark cost unknown, continue.

## 8. Self-Test

Per Engineering Law 11:

- Each of 3 contract JSONs parses against `AIOS-SCHEMA-000015`.
- Same mock demand → same `plan_id` (determinism fixture).
- Mock irreversible step → `blocking_steps[]` contains it.
- Mock over-budget step → `budget_violations[]` contains it.
- Vague demand → PLANNER_GOAL_TOO_VAGUE emitted.
- Gap demand → PLANNER_CAPABILITY_GAP emitted.

Test entry: `aios/planner/tests/planner-selftest.json`.