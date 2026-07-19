# Project Understanding — Query Algorithms

## Algorithm: Symbol Resolution
```
input: query string "validateToken"
1. search all symbol nodes for name ~= "validateToken"
2. filter by type in ["function", "method"]
3. return top 5 matches sorted by: exact match > prefix match > substring match
4. include file path and line number for each match
```

## Algorithm: Call Graph
```
input: symbol_id "sym://AuthController.login"
1. find all edges of type "calls" where from = symbol_id
2. for each target, recursively find what it calls (depth = 2)
3. return directed graph of calls
```

## Algorithm: Dependency Impact
```
input: file_path "src/auth/AuthService.ts"
1. find all edges of type "imports" where from = file_path
2. find all edges of type "imports" where to = file_path (reverse)
3. return { imports: [...], imported_by: [...] }
```

## Confidence Scoring
```
confidence = (index_uptodate ? 1.0 : 0.5) * (match_quality) * (1.0 - staleness_penalty)
```