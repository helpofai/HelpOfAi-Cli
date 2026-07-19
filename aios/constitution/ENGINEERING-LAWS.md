# Engineering Laws of AIOS v1.0

Universal laws that every AIOS engine, agent, and workflow must follow.
These are not suggestions. These are the operating system's immutable rules.

---

## Law 1: Analyze Before Write
Never modify files before analysis is complete and approved.
- What is the current state?
- What depends on this?
- What is the impact of the change?
- What is the rollback plan?

## Law 2: Search Before Create
- Check if the capability already exists.
- Check if the function/class/component already exists.
- Check if the file is already generated.

## Law 3: Architecture First
- Allocate significant effort upfront for architecture design and analysis before any code generation.
- Architecture decisions are recorded and reasoning is explained.

## Law 4: Rollback Always
- Every change has a documented rollback strategy before execution.
- Files are never overwritten without a backup strategy.

## Law 5: Verify Everything
- Every module is validated after creation.
- Manifest validation, dependency validation, schema validation.
- Integrity checking at every installation and update.

## Law 6: Document Automatically
- Code generation includes inline documentation.
- Feature creation includes specification updates.
- Every decision logged in the decision journal.

## Law 7: Preserve Intent
- Follow existing conventions and patterns.
- Respect the codebase's existing structure.
- Never reformat unrelated code.

## Law 8: Cache Aggressively
- Build results are cached.
- Analysis results are cached.
- Dependency graphs are cached with incremental invalidation.

## Law 9: Fail Gracefully
- When a module fails, the system continues.
- Errors are logged with context.
- Recovery suggestions are provided to the operator.

## Law 10: Clean Up
- Temporary files are removed after use.
- Old cache entries are pruned.
- Orphaned dependencies are detected and cleaned.

## Law 11: Profile First, Optimize Second
- Measure performance before optimizing.
- Declare performance budgets.
- Never optimize at the cost of correctness or readability.

## Law 12: Internationalization Ready
- All external-facing strings are prepared for localization.
- Internal constants use English.
- No hardcoded locale-dependent formatting.

## Law 13: Complete Traceability
- Feature → Plan → Implementation → Validation → Test → Review → Deployment.
- Every commit, every file, every change traceable to origin.

## Law 14: Knowledge Persistence
- Project understanding persists across sessions.
- Cached brain data persists with incremental updates.
- No re-analysis of unchanged code.

## Law 15: Session Hygiene
- Temporary files cleaned after task completion.
- Context isolation for parallel agents.
- Clean separation between sessions.

## Law 16: Immutable Core
- Core subsystems (constitution, kernel, runtime) cannot be modified by plugins.
- Extensions must use the public API and event hooks.
- Plugin failures cannot crash the kernel.

---

## Consequence of Violation

An AIOS subsystem that violates an engineering law:
1. Logs a constitutional violation.
2. Reverts to the previous state where possible.
3. Reports the violation for audit.
4. Continues with degraded capability for non-critical violations.

Critical violations (corruption, data loss) trigger `hoa repair` recovery mode.