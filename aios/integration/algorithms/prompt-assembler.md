# Prompt Assembler — Algorithm

## Assembly Sequence
```
1. Determine target agent/engine from workflow phase
2. Load base prompt template (from agent's prompts/ or engine's prompts/)
3. Inject context from brain:
   - If brain is indexed: inject relevant_file_context, project_summary
   - If brain is not indexed: inject basic project structure
4. Inject plan context:
   - Current phase description + inputs from prior phases
   - Gate conditions for current phase
   - Rollback instructions
5. Inject constitutional constraints:
   - Ground truth rule (Constitution I)
   - Verification rule (Constitution II)
   - Act, don't ask rule
   - Scope discipline rule
6. Truncate to fit model token budget (agent-runtime-configs.json)
7. Return: assembled_prompt (string)
```

## Priority Order
```
1. Operator overrides (if any)
2. Project instructions from .helpofai
3. Agent's own prompt template
4. Constitutional constraints
5. Brain context
6. Plan context
```