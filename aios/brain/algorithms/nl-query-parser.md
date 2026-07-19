# Natural Language Query Parser

## Purpose
Parse free-text queries into structured brain queries.

## Algorithm
```
1. Tokenize query into words
2. Classify query intent using keyword matching:
   - "callers|who calls|what uses" → CALL_GRAPH query
   - "imports|depends|dependency" → DEPENDENCY query
   - "features|what features" → FEATURE query
   - "files changed|modified since" → CHANGE query
   - "similar|duplicate|same as" → SIMILARITY query
3. Extract entities:
   - Quoted strings → exact symbol/file names
   - CamelCase/PascalCase → symbol names
   - Path-like strings → file paths
4. Build structured query:
   {
     intent: "CALL_GRAPH",
     target: "AuthService",
     depth: 2,
     filters: { type: "function" }
   }
```

## Intent Examples
| Query | Intent | Target |
|-------|--------|--------|
| "who calls AuthService.validateToken" | CALL_GRAPH | AuthService.validateToken |
| "what imports the User model" | DEPENDENCY | User |
| "what features use auth" | FEATURE_QUERY | auth |
| "files modified since last release" | CHANGE_QUERY | (all) |
| "find similar files to AuthController" | SIMILARITY | AuthController |