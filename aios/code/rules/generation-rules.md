# Code Engine — Generation Rules

## File Creation Rules
```
- Always include file header with module name, purpose, and creation date
- Follow existing project naming conventions (detected from package files)
- Generate imports/exports automatically from usage
- Never modify files outside the declared scope
```

## Edit Rules
```
- Prefer append over modify (add new functions at end of file)
- When modifying existing code: use minimal diff, preserve existing style
- Always generate a rollback script alongside destructive changes
- Never modify files in kernel/ or constitution/ directories
```

## Output Format
```
changes: [
  {
    file_path: "src/auth/AuthController.ts",
    operation: "create",  // create | edit | delete
    content: "...",       // full file content for create, diff for edit
    trace_back: {
      plan_step_id: "step-003",
      capability_id: "AIOS-CAPABILITY-000060",
      feature_id: "AIOS-FEATURE-000042"
    }
  }
]
```