# Engines — Test Fixtures

## Analysis Engine
```
INPUT: module_id = "AIOS-MODULE-000002", brain snapshot with 5 layer violations
EXPECTED: architecture_score < 60, risks contains "layer violation"
```

## Code Engine
```
INPUT: plan_step = "add auth controller", feature_spec for auth
EXPECTED: changes[0].operation = "create", changes[0].file_path matches "*AuthController*"
```

## Testing Engine
```
INPUT: changed_files = ["src/auth/AuthController.ts"], project_root = "/test-project"
EXPECTED: test_results.total > 0, framework detected as jest/vitest/pytest
```

## Review Engine
```
INPUT: changes from code_engine output with security vulnerability
EXPECTED: review.comments contains critical severity, review.pass = false
```

## Bug Engine
```
INPUT: evidence = "TypeError: Cannot read property 'email' of null at AuthService.ts:42"
EXPECTED: analysis.root_causes[0].file contains "AuthService.ts", severity = "high"
```

## Security Engine
```
INPUT: manifest_path = "package.json" with vulnerable dependency
EXPECTED: vuln_report.critical_count > 0, remediation suggested
```

## DevOps Engine
```
INPUT: execution_plan for code change, environments = ["staging"]
EXPECTED: deployment_plan.deploy_steps contains canary rollout
```

## Performance Engine
```
INPUT: target = "src/api/users.ts" with N+1 query pattern
EXPECTED: perf_report.bottlenecks[0].metric = "n_plus_1"
```