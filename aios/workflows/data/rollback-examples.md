# Workflow — Rollback Examples

## Example 1: Code Generation Rollback
```
Changes made: /src/auth/AuthController.ts (new), /src/auth/AuthService.ts (modified)
Rollback:
  1. Revert AuthService.ts to HEAD
  2. Delete AuthController.ts
  3. Verify: git diff --stat shows no net change
```

## Example 2: Migration Rollback
```
Changes made: 2026_07_19_add_email_verified_to_users.php (migration run)
Rollback:
  1. Run migrate:rollback once
  2. Verify: migration table shows no entry for 2026_07_19
```

## Example 3: Config Rollback
```
Changes made: .env.example (modified), config/auth.php (modified)
Rollback:
  1. Revert .env.example from git HEAD
  2. Revert config/auth.php from git HEAD
  3. Verify: files match HEAD
```

## Rollback Script Structure
Each workflow generates:
```
.rollback/
├── rollback-{workflow_id}.sh    → executable rollback script
├── undo-{file_path}.patch       → inverse patch per file
└── verify-{workflow_id}.sh      → post-rollback verification
```