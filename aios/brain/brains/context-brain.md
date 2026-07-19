# Context Brain — AIOS-BRAIN-000004

Assembles the minimal context needed for a request. Queries the other brains
for relevant slices — project map for files matching the request, feature brain
for relevant features, dependency brain for affected modules. This is how AIOS
avoids loading the entire project into context.

### How it works
1. Request parsed by kernel (AIOS-CONTRACT-000010)
2. Context Brain selects relevant brain slices by keyword/embedding match
3. Returns a context_pack with token estimate
4. Kernel routes using only the packed context