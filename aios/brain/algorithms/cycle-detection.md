# Dependency Cycle Detection — Tarjan's Algorithm

## Purpose
Detect circular dependencies in the module/symbol dependency graph.

## Algorithm (Tarjan's SCC)
```
1. Assign each node a unique index (DFS order)
2. Maintain a stack of nodes in the current DFS path
3. For each node:
   a. Set node.index = node.lowlink = current_index++
   b. Push node onto stack
   c. For each neighbor (dependency):
      - If neighbor not visited: recursively visit, then node.lowlink = min(node.lowlink, neighbor.lowlink)
      - If neighbor on stack: node.lowlink = min(node.lowlink, neighbor.index)
   d. If node.lowlink == node.index:
      - Pop nodes from stack until node is popped → this is a strongly connected component
      - If SCC has >1 node: CYCLE DETECTED
      - If SCC has 1 node that depends on itself: SELF-LOOP DETECTED
4. Return all cycles with their node paths
```

## Integration
- Runs on every dependency graph update
- Cycles with >3 nodes are flagged as CRITICAL
- Self-loops are flagged as WARNING
- Results cached until next graph update