# Integration Scenario: Profile Switch

## Context
Operator switches from "default" to "minimal" to reduce load.

## Steps
1. Operator: `hoa profile use minimal`
2. Integration module:
   - Reads `data/profiles/minimal.json`
   - Unloads all modules not in the minimal set:
     - Brain unloaded → brain cache cleared
     - All engines unloaded → engine caches cleared
     - Agents unloaded → agent pool reset
     - Workflows unloaded
   - Verifies kernel + runtime + planner remain loaded
3. System reports:
   - 3 modules loaded (down from 25)
   - 3 capabilities available (down from 34)
   - Some commands unavailable: build-feature, fix-bug, review

## Reversal
1. Operator: `hoa profile use default`
2. Integration module re-loads all 25 modules
3. Brain needs re-index → `hoa brain index`