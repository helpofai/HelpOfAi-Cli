# Digital Twin — Conflict Scenarios

## Scenario 1: Same File, Different Sections
Plan A edits `auth.ts:15-30`. Plan B edits `auth.ts:50-70`.
→ No overlap. Auto-merge.

## Scenario 2: Same File, Same Lines
Plan A changes `auth.ts:15-30`. Plan B changes `auth.ts:20-25`.
→ Overlap. Block both. Show diffs. Operator must resolve.

## Scenario 3: Symbol Conflict
Plan A renames `loginUser()` to `authenticateUser()`. Plan B adds a call to `loginUser()`.
→ Symbol conflict. Flag with both plan IDs. Suggest rename plan runs first.

## Scenario 4: Chain Conflict
Plan A deletes `database/migrations/`. Plan B modifies a migration file.
→ Dependency conflict. Plan A must complete before Plan B can be assessed.

## Resolution Strategies
- **Reorder**: Run conflicting plans sequentially instead of parallel
- **Merge**: Combine both changes into one plan
- **Abort**: Abandon one plan, keep the other