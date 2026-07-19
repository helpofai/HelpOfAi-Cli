# Agent Tool Bindings

## Master Agent (AIOS-MASTER-000001)
```
exec_shell → full
read_file → full
write_file → full
apply_patch → full
grep_files → full
git_diff → full
agent → orchestrator only (can spawn sub-agents)
```

## Architect Agent (AIOS-MASTER-000002)
```
read_file → full
grep_files → full
agent → restricted (analyze + plan only)
```

## Backend Agent (AIOS-MASTER-000003)
```
exec_shell → full
read_file → full
write_file → full
apply_patch → full
grep_files → full
```

## QA Agent (AIOS-MASTER-000007)
```
read_file → full
grep_files → full
exec_shell → restricted (test commands only)
run_tests → full
```

## Security Agent (AIOS-MASTER-000009)
```
read_file → full
grep_files → full
exec_shell → restricted (security commands only)
```