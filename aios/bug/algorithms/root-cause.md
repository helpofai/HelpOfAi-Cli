# Bug Engine — Root Cause Analysis Algorithm

## Algorithm
```
1. Normalize evidence:
   - Parse stack trace for file:line references
   - Extract error message and error code
   - Identify the throwing function/symbol

2. Score candidate locations:
   for each candidate in trace:
     score = (frequency_in_brain * 0.3) + (recent_changes * 0.3) + (error_similarity * 0.4)
   
3. Rank candidates by score:
   - Top candidate: most likely root cause
   - Show top 3 candidates with confidence scores

4. Generate fix suggestion:
   - For known bug patterns: suggest specific fix
   - For unknown patterns: suggest investigation approach
```

## Known Bug Patterns
| Pattern | Detection | Fix |
|---------|-----------|-----|
| Null reference | Trace contains "Cannot read property of null" | Add null check |
| Missing import | "module not found" in trace | Add import statement |
| Type mismatch | "Type X is not assignable to type Y" | Add type conversion |
| API timeout | "timeout" in trace | Add retry logic |
| Race condition | Intermittent failures in async code | Add mutex/lock |