# Agent System — Test Fixtures

## Test: Agent Dispatch
```
INPUT: request = "build feature auth"
EXPECTED: master agent is selected, architect and backend are spawned
```

## Test: Tool Binding Enforcement
```
INPUT: review agent requests exec_shell
EXPECTED: access denied (review agent has no exec_shell permission)
```

## Test: Capability Routing
```
INPUT: request = "fix bug in login code"
EXPECTED: master agent routes to bug agent (AIOS-MASTER-000016)
```

## Test: Agent Isolation
```
INPUT: agent_a modifies file → agent_b reads file
EXPECTED: agent_b sees agent_a's changes if within project scope
```

## Test: Agent Timeout
```
INPUT: agent exceeds max_steps (10) without completing
EXPECTED: agent is terminated, error reported to operator
```

## Test: Concurrent Agents
```
INPUT: architect + qa agents spawned simultaneously
EXPECTED: both complete, results merged by master
```