# Learning Engine — Example Proposals

## Example 1: Bug Pattern
```
Proposal: NEW_RULE-001
Type: new_rule
Description: "API route `/users` missing input validation — detected 5 times in 30 days"
Confidence: 0.92
Impact: Prevents future validation bugs
Suggested: Add route validation middleware rule
```

## Example 2: Workflow Bottleneck
```
Proposal: WORKFLOW_CHANGE-001
Type: workflow_change
Description: "Testing phase fails gate 35% of the time (threshold: 20%)"
Confidence: 0.85
Impact: Reduces rework by 15%
Suggested: Add pre-test lint gate before running tests
```

## Example 3: Knowledge Gap
```
Proposal: KNOWLEDGE_ENTRY-001
Type: knowledge_entry
Description: "Operator clarified error format 3 times in 2 weeks"
Confidence: 0.78
Impact: Reduces clarification requests
Suggested: Add "API error format convention" to Knowledge Base
```