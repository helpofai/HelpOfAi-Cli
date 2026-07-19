# Feature Impact Analysis — BFS Algorithm

## Purpose
When a feature is modified, predict which other features, files, and modules are affected.

## Algorithm (Breadth-First Search)
```
1. Start node: feature_id or file_path of the changed feature
2. BFS traversal:
   a. Queue = [start_node]
   b. Visited = {start_node}
   c. Impact = {direct: [], indirect: [], cascading: []}
   d. While queue is not empty:
      - current = queue.dequeue()
      - For each edge (current → neighbor):
         - If neighbor not visited:
            - Mark as visited
            - If depth == 0: impact.direct.append(neighbor)
            - If depth == 1: impact.indirect.append(neighbor)
            - If depth >= 2: impact.cascading.append(neighbor)
            - Queue neighbor
3. Return impact report with depth levels
```

## Depth Thresholds
- Direct (depth 0): immediate dependents — must be re-verified
- Indirect (depth 1): dependents of dependents — may need attention
- Cascading (depth 2+): distant dependents — low priority unless critical

## Example
```
Feature A → Feature B → Feature C → Feature D
Change A:
  Direct: [B]
  Indirect: [C]
  Cascading: [D]
```