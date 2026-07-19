# Brain — Test Fixtures

## Test: File Indexing — New File
```
INPUT: project_root="/tmp/test-project", changed_files=["src/new.ts"]
EXPECTED: graph_delta.files_indexed = 1, graph_version incremented
```

## Test: File Indexing — No Changes
```
INPUT: project_root="/tmp/test-project", changed_files=[]
EXPECTED: graph_delta.files_indexed = 0, graph_version = same
```

## Test: Project Understanding — Symbol Query
```
INPUT: query="AuthController", brain has indexed auth module
EXPECTED: answer.nodes contains "sym://AuthController.login", "sym://AuthController.register"
```

## Test: Project Understanding — Unknown Symbol
```
INPUT: query="NonExistentSymbol"
EXPECTED: answer.nodes = [], answer.edges = [], notes contains "no match found"
```

## Test: Staleness Detection
```
INPUT: graph_version = 5, latest_file_modification > graph.last_indexed_at
EXPECTED: BRAIN_STALE flag raised
```

## Test: Context Pack Assembly
```
INPUT: request_text="add login page"
EXPECTED: context_pack.relevant_files contains auth-related files
EXPECTED: context_pack.estimated_tokens < 5000
```