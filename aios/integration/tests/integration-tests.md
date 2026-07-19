# Integration — Test Fixtures

## Loader Tests
```
Test: Full load (28 modules)
INPUT: profile = default
EXPECTED: loaded = 28, failed = 0

Test: Missing dependency  
INPUT: module X depends on MODULE_MISSING
EXPECTED: loaded = false, error = "MODULE_MISSING not found"

Test: Schema validation failure
INPUT: module.json with missing "id"
EXPECTED: loaded = false, error = "schema validation failed"
```

## Registry Tests
```
Test: Resolve capability
INPUT: cap_id = "AIOS-CAPABILITY-000010"
EXPECTED: module_id = "AIOS-MODULE-000002", name = "request_routing"

Test: Unknown capability
INPUT: cap_id = "AIOS-CAPABILITY-999999"
EXPECTED: error = "CAPABILITY_NOT_FOUND"
```

## Workflow Builder Tests
```
Test: Build intent
INPUT: request = "build auth feature"
EXPECTED: workflow = "build-feature", confidence > 0.5

Test: Fix intent
INPUT: request = "fix login bug"
EXPECTED: workflow = "fix-bug", confidence > 0.5

Test: Unknown intent
INPUT: request = "do something random"
EXPECTED: error = "INTENT_UNKNOWN"
```

## Prompt Assembler Tests
```
Test: Basic assembly
INPUT: agent_id = "AIOS-AGENT-000003"
EXPECTED: prompt contains "Constitution" and "Verification"

Test: Token budget
INPUT: agent with 2000 max_tokens
EXPECTED: assembled prompt <= 2000 tokens
```