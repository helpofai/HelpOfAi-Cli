# Context Relevance Scoring — Multi-Factor Model

## Purpose
Score how relevant each brain node is to a given user request, for context pack assembly.

## Factors
| Factor | Weight | Computation |
|--------|--------|-------------|
| keyword_match | 0.30 | TF-IDF score of request terms against node content |
| recency | 0.15 | 1.0 if modified in last day, decays to 0.0 over 30 days |
| dependency_centrality | 0.20 | PageRank of the node in the dependency graph |
| user_frequency | 0.10 | How often the user has referenced this node |
| feature_priority | 0.15 | Priority of the feature this node belongs to |
| error_rate | 0.10 | Inverse of bug count in this node (fewer bugs = higher score) |

## Final Score
```
relevance = sum(factor_i * weight_i) * recency_multiplier
normalized to 0.0 - 1.0
```

## Thresholds
- relevance >= 0.6: include in context pack (highly relevant)
- 0.3 <= relevance < 0.6: include if budget allows
- relevance < 0.3: exclude from context pack

## Budget-Aware Assembly
```
estimated_tokens = sum(node.estimated_tokens for each included node)
while estimated_tokens > max_context_tokens:
    drop lowest-relevance node
    recalculate
```

## Default Max Context
- Standard mode: 4000 tokens
- Deep mode (`--deep`): 8000 tokens
- Quick mode (`--quick`): 1500 tokens