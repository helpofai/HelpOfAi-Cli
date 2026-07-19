# Workflow Builder — Implementation Spec

## Core Function
```rust
fn build_workflow(request: &str, ctx: &Context) -> Result<WorkflowInstance, BuilderError>
```

## Intent Classification
```rust
struct IntentClassifier {
    patterns: Vec<IntentPattern>,
}

struct IntentPattern {
    keywords: Vec<String>,
    workflow_id: String,
    confidence: f64,
}

fn classify(request: &str) -> (String, f64) {
    // 1. Tokenize request (lowercase, split on spaces/commas)
    // 2. For each pattern, count keyword matches
    // 3. Best match = pattern with highest keyword overlap
    // 4. Confidence = match_count / pattern.keywords.len()
    // 5. Return (workflow_id, confidence)
}
```

## Workflow Template
```rust
struct WorkflowTemplate {
    id: String,
    name: String,
    phases: Vec<PhaseTemplate>,
    rollback_policy: String,
}

struct PhaseTemplate {
    id: String,
    agent_id: Option<String>,
    engine: Option<String>,
    gate: Option<String>,
    timeout_s: u32,
    retry: u32,
    parallel: bool,
}
```

## Error Handling
```rust
enum BuilderError {
    IntentUnknown(String),           // no pattern matched
    WorkflowNotFound(String),        // matched but no template
    TemplateInvalid(String),         // failed to parse template
}
```