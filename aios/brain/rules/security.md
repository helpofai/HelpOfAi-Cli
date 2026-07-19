# Knowledge Graph — Security Rules

## Access Control
- Brain data is readable by all modules
- Only kernel and planner may modify the dependency graph
- Only the analysis engine may add security annotations
- All other modules are read-only consumers

## Sensitive Data
- Files matching `**/env`, `**/secret*`, `**/credential*` are flagged
- Sensitive files are recorded as nodes but their contents are NOT indexed
- Symbol extraction is skipped for sensitive files
- A permissions check runs before indexing each file:
  ```
  if file matches sensitive_pattern:
      skip_symbol_extraction = true
      flag_as_sensitive = true
  ```

## Audit Trail
- Every graph modification is logged with: modifier, timestamp, change_type
- Use `hoa brain debug --audit-log` to view modification history
- Audit history is append-only (Engineering Law 13)