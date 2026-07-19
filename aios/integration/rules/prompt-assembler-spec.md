# Prompt Assembler — Implementation Spec

## Core Function
```rust
fn assemble_prompt(
    agent_id: &str,
    plan_context: &PlanContext,
    brain_context: &BrainContext,
) -> Result<AssembledPrompt, AssemblerError>
```

## Context Injection Order
```rust
fn assemble(agent_id, plan_ctx, brain_ctx) -> String {
    let mut prompt = String::new();
    
    // Layer 1: Constitution Sections
    prompt.push_str(&load_constitution_rules(agent_id));
    
    // Layer 2: Agent Base Prompt
    prompt.push_str(&load_agent_prompt(agent_id));
    
    // Layer 3: Project Context
    if let Some(proj) = &brain_ctx.project_summary {
        prompt.push_str(&format!("\n## Project Context\n{}\n", proj));
    }
    
    // Layer 4: Plan Context
    prompt.push_str(&format!("\n## Task\n{}\n", plan_ctx.description));
    if let Some(inputs) = &plan_ctx.inputs {
        prompt.push_str(&format!("\n## Inputs\n{}\n", inputs));
    }
    
    // Layer 5: Gate Constraints
    if let Some(gate) = &plan_ctx.gate {
        prompt.push_str(&format!("\n## Gate Condition\n{}\n", gate));
    }
    
    // Layer 6: Relevant Files from Brain
    if let Some(files) = &brain_ctx.relevant_files {
        let files_str = files.join("\n");
        prompt.push_str(&format!("\n## Relevant Files\n{}\n", files_str));
    }
    
    prompt
}
```

## Token Budget Enforcement
```rust
fn truncate_to_budget(prompt: &str, max_tokens: u32) -> String {
    let tokens = count_tokens(prompt);
    if tokens <= max_tokens {
        return prompt.to_string();
    }
    // Remove brain context first (lowest priority)
    // Then plan context
    // Keep constitution + agent prompt (highest priority)
    truncate_lowest_priority(prompt, max_tokens)
}
```