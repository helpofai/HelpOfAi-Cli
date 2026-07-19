# Conflict Detection Rules

## Level 1 — File Conflict
Two plans modify the same file → flag with both plan IDs and overlapping lines.

## Level 2 — Symbol Conflict
Two plans modify the same symbol (class, function, route) even in different files → flag with symbol details.

## Level 3 — Dependency Conflict
Plan A deletes a dependency that Plan B depends on → flag with dependency chain.

## Resolution
- If conflict probability > 0.8, block both plans until resolved.
- If 0.4 < probability < 0.8, warn operator with options: merge, reorder, split.
- If < 0.4, log and proceed.