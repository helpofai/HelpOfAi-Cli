# Prompt Assembler — CLI Integration Spec

The Assembler builds LLM prompts from agent definitions, brain data, and templates.

## Assembly Flow

```
User Request
  ↓
Kernel (request_routing) → CapabilityDemand
  ↓
Context Brain → context_pack (relevant brain slices)
  ↓
Planner → ExecutionPlan
  ↓
For each plan step:
  ↓
Select Agent (by capability match)
  ↓
Load Agent Prompt Template: aios/agents/prompts/AGENT-NNNNNN-*.md
  ↓
Inject context: brain slices + plan step + project conventions
  ↓
Assemble final prompt → submit to LLM
```

## Prompt Structure

Every prompt follows this structure:
1. **System prefix** — from the agent's prompt template (role, authority, standards)
2. **Context** — brain slices (project map, relevant files, deps)
3. **Task** — the plan step to execute
4. **Output format** — the expected response structure
5. **Constraints** — boundaries, quality standards

## Template Variables

Prompt templates use `{{mustache}}` placeholders:
- `{{project_context}}` — brain slices
- `{{plan_step}}` — the current execution step
- `{{conventions}}` — framework/language conventions from packs
- `{{constraints}}` — hard constraints from the request