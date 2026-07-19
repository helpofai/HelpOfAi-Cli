# Workflow Builder — CLI Integration Spec

The CLI builds executable workflows from workflow JSON definitions.

## Process

1. On `hoa build feature` (or any trigger match), find the matching workflow
   in `aios/workflows/WORKFLOW-NNNNNN-*.json`.
2. Read the `lifecycle` array — ordered phases with engine assignments.
3. For each phase, resolve the engine module via `capabilities.json`.
4. If the engine module isn't loaded, load it via the Module Loader.
5. Execute phases in order. If a phase's `gate` fails, run `rollback_workflow`.
6. Each phase emits events (`phase_started`, `phase_completed`) on the event bus.

## Template Rendering

After a phase completes, the CLI can render output using templates from
`aios/templates/`. For example, after the Planning phase, render `PLAN.md`
with plan variables substituted.

## Example

```
hoa build auth-feature
  → matches WORKFLOW-000001-build-feature.json
  → phases: understand→analyze→plan→implement→validate→review
  → runs each phase through its assigned engine
  → gates: tests_pass, review_approved
  → outputs: EXECUTION_REPORT.md with results
```