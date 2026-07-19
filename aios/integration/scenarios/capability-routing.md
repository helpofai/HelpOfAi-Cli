# Integration Scenario: Capability Routing

## Context
A workflow phase needs capability AIOS-CAPABILITY-000060 (code_generation).

## Flow
1. Workflow sends request: `{capability_id: "AIOS-CAPABILITY-000060", inputs: {...}}`
2. Kernel receives request → queries registry reader
3. Registry reader resolves: `AIOS-CAPABILITY-000060 → AIOS-MODULE-000009 (code)`
4. Kernel checks: module AIOS-MODULE-000009 is loaded? YES
5. Kernel routes to code engine: `POST /code/generate`
6. Code engine processes → returns result
7. Kernel returns result to workflow

## Error Flow
1. Request: `AIOS-CAPABILITY-000060`
2. Registry reader resolves: module found
3. Kernel checks: module NOT loaded
4. Kernel returns: `{error: "MODULE_NOT_LOADED", recovery: "hoa module load AIOS-MODULE-000009"}`

## Multiple Providers
1. Request: `AIOS-CAPABILITY-000100` (provided by 2 modules)
2. Registry reader finds: [AIOS-MODULE-000007, AIOS-MODULE-000011]
3. Kernel selects: first loaded (or first in list if both loaded)
4. Routes to selected module