# Testing Engine — Test Discovery Algorithm

## Framework Detection
```
1. Check project root for config files:
   - jest.config.* → jest
   - vitest.config.* → vitest
   - pytest.ini, tox.ini → pytest
   - Cargo.toml → cargo test
   - go.mod → go test
2. If multiple detected: use the most specific one
3. If none detected: return TEST_FRAMEWORK_UNKNOWN
```

## Test Selection
When given a set of changed files, select relevant tests:
```
1. For each changed file, find its test file:
   - src/auth/AuthController.ts → tests/auth/AuthController.test.ts
   - src/auth/AuthController.ts → src/auth/__tests__/AuthController.test.ts
   - src/auth/AuthController.ts → tests/AuthControllerTest.php
2. Include integration tests that import the changed module
3. Include E2E tests tagged with the affected feature
4. Run selected tests + their dependencies
```

## Coverage Calculation
```
coverage = (covered_lines / total_executable_lines) * 100
delta = new_coverage - previous_coverage
```