# Workflow Builder — Algorithm

## Build Sequence
```
1. Parse user request: tokenize, extract intent and entities
2. Match intent to workflow type via keyword classification:
   - "build|create|add feature" → build-feature
   - "fix|bug|error" → fix-bug
   - "review|inspect" → review-code
   - "refactor|restructure" → refactor
   - "optimize|perf" → optimize
   - "release|deploy" → release
3. Load workflow template (from aios/workflows/WORKFLOW-NNNNNN-*.json)
4. Fill template inputs:
   - description: extracted from request
   - target files: resolved via brain (if applicable)
   - auto_confirm: default true, overridable
5. Return: workflow_definition + filled_inputs
```

## Intent Classification Table
| Keywords | Workflow | Confidence |
|----------|----------|-----------|
| build, create, add, feature, implement | build-feature | 1.0 |
| fix, bug, broken, error, crash, issue | fix-bug | 1.0 |
| review, inspect, check, audit, verify | review-code | 1.0 |
| refactor, restructure, reorganize | refactor | 0.9 |
| optimize, performance, speed up | optimize | 0.9 |
| release, deploy, ship, version | release | 1.0 |