# Agent System — Common Scenarios

## Scenario 1: New Project
1. Operator: "build new auth service"
2. Master dispatches architect → design
3. Architect returns plan with 5 steps
4. Master dispatches backend + database + qa
5. All complete → master synthesizes → operator review

## Scenario 2: Bug Triage
1. Operator: "login failing with OAuth"
2. Master dispatches backend + qa
3. Backend finds root cause (config mismatch)
4. QA runs regression tests
5. Results synthesized to operator

## Scenario 3: Production Emergency
1. Operator: "production is down!"
2. Master dispatches security + devops + backend
3. Security checks for breach: none
4. DevOps checks deployment: recent change
5. Backend confirms: bug in release
6. DevOps rolls back → system restored → 12 min MTTR