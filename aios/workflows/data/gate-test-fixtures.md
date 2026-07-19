# Workflow — Gate Test Fixtures

## Test: Quality Gate Pass
```
INPUT: gate = "quality", architecture_score = 72, security_score = 85
EXPECTED: gate_passed = true, score = 78.5
```

## Test: Quality Gate Fail
```
INPUT: gate = "quality", architecture_score = 45, security_score = 50
EXPECTED: gate_passed = false, score = 47.5
```

## Test: Review Gate Pass
```
INPUT: gate = "review", critical = 0, major = 1
EXPECTED: gate_passed = true
```

## Test: Review Gate Fail
```
INPUT: gate = "review", critical = 1, major = 2
EXPECTED: gate_passed = false
```

## Test: Test Gate Pass
```
INPUT: gate = "test", total = 24, passed = 24, failed = 0
EXPECTED: gate_passed = true
```

## Test: Test Gate Fail
```
INPUT: gate = "test", total = 24, passed = 22, failed = 2
EXPECTED: gate_passed = false
```

## Test: Manual Gate
```
INPUT: gate = "manual"
EXPECTED: gate_passed = null, status = "suspended" (awaiting operator)
```