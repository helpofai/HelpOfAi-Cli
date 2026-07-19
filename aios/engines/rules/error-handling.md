# Engines — Error Handling Reference

## Analysis Engine
```
ANALYSIS_MODULE_NOT_FOUND → report partial, list installed
ANALYSIS_BRAIN_STALE → best-effort scoring, flag staleness
ANALYSIS_TOO_LARGE → partial coverage, suggest focused target
```

## Code Engine
```
CODE_NOTHING_CHANGED → empty changes, no empty files
CODE_FILE_EXISTS → suggest edit instead of create
CODE_READ_ONLY → refuse to modify immutable modules
```

## Testing Engine
```
TEST_FRAMEWORK_UNKNOWN → empty result, no guess
TEST_TIMEOUT → partial results, flag timeout
TEST_NO_TESTS_FOR_CHANGES → empty result, not an error
```

## Review Engine
```
REVIEW_DIFF_PARSE_FAILED → skip file, continue
REVIEW_TOO_MANY_COMMENTS → top N by severity, rest bucketed
```

## Bug Engine
```
BUG_NO_CANDIDATES → insufficient evidence, request more info
```

## Security Engine
```
SECURITY_MANIFEST_NOT_FOUND → empty report
SECURITY_CVE_DATABASE_OFFLINE → local cached snapshot, flag staleness
SECURITY_NO_POLICY → best-effort listing
```

## DevOps Engine
```
DEVOPS_ENVIRONMENT_UNKNOWN → suggest creating profile
DEVOPS_PLATFORM_UNSUPPORTED → list supported platforms
```

## Performance Engine
```
PERF_NO_DATA → insufficient profile data
PERF_BUDGET_OK → no violations found
```