# Capability Graph — Mermaid Export Template

```mermaid
graph LR
  A[request_routing] -->|routes_to| B[goal_decomposition]
  B --> C[code_generation]
  C --> D[unit_testing]
  D --> E[code_review]

  style A fill:#4a9,color:#fff
  style B fill:#49a,color:#fff
  style C fill:#49a,color:#fff
  style D fill:#49a,color:#fff
  style E fill:#a94,color:#fff
```

Use with: `hoa cap-graph --export mermaid`