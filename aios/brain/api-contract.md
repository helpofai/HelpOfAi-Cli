# Brain — API Contract Reference

## Public API

### file_indexing (AIOS-CONTRACT-000041)
```
POST /brain/index
{
  "project_root": "/workspace",
  "changed_files": ["src/auth.ts"]  // optional, null = full re-index
}
→ {
  "graph_delta": { "files_indexed": 1, "symbols_added": 12, "graph_version": 43 }
}
```

### project_understanding (AIOS-CONTRACT-000040)
```
POST /brain/query
{
  "query": "callers of AuthService",
  "since_commit": null  // optional
}
→ {
  "nodes": [...],
  "edges": [...],
  "notes": "Fresh index, 5 results"
}
```

### context_pack (internal, no public contract)
```
POST /brain/context
{
  "request_text": "build auth feature",
  "depth": "standard"
}
→ {
  "relevant_files": [...],
  "estimated_tokens": 1200
}
```

## Error Responses
All endpoints return:
```json
{
  "error": {
    "code": "BRAIN_NOT_INDEXED",
    "message": "No knowledge graph exists. Run `hoa brain index` first.",
    "recovery": "hoa brain index"
  }
}
```