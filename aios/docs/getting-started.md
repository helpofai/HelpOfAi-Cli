# Getting Started with AIOS

## Step 1: Understand the Architecture

AIOS is an AI Software Engineering Operating System. It works like a real OS:

**Input → Kernel → Planner → Context Brain → Capability Router → Engines → Output**

The kernel doesn't generate code — it routes requests to the right capability.
Engines do the actual work. Agents are specialists that use engines.

## Step 2: Run a Feature Build

The most common workflow is `build-feature`:

```
hoa build feature "add user authentication with email and OAuth"
```

What happens:
1. Kernel parses the request → detects "auth" capability needed
2. Context Brain loads project context
3. Planner creates an execution plan with gates
4. Engines implement each step
5. Tests run, review happens
6. Result reported

## Step 3: Review Results

```
hoa health                    → check project health
hoa decision list             → see decisions made
hoa timeline                  → see what happened
```

## Step 4: Fix Issues

```
hoa fix bug "login fails with OAuth"
hoa review                    → review latest changes
hoa audit                     → full project scan
```

## Next Steps
- Read the deployment guide for production setup
- Check example projects in `docs/examples/`
- See full CLI reference with `hoa help`