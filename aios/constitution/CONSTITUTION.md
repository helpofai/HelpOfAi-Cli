# HelpOfAi AIOS Constitution v1.0.0

> The immutable foundation of the AI Software Engineering Operating System.
> All subsystems, engines, agents, and plugins derive their authority from this document.

## I. Design Principle

HelpOfAi AIOS is a modular, offline-first, deterministic, enterprise-grade AI Software
Engineering Operating System. It builds maintainable, secure, production-ready software
through transparent reasoning, versioned knowledge, reusable engineering workflows, and
independently loadable modules.

---

## II. Golden Rule

AIOS must optimize for correctness, maintainability, and long-term value over speed alone.

---

## III. Core Principles

### 1. Capability-First Architecture
- AIOS loads **only the capabilities needed** for each request.
- The CLI manages the directory tree at rest; AIOS retrieves by capability at runtime.
- Rule: **90% routing, 10% generation.**

### 2. Offline-First
- AIOS works without internet access.
- Local models, local schemas, local registries are the default path.
- Cloud is an extension, not a requirement.

### 3. Modular & Decoupled
- Every subsystem is independently loadable.
- No subsystem depends on all others.
- Dependencies are explicit: `requires`, `optional`, `conflicts`, `provides`.

### 4. Deterministic
- Same project + same AIOS version + same config → reproducible engineering plan.
- Non-deterministic behavior is explicitly marked.

### 5. Machine-First Metadata
- Every human-facing document has a machine-readable companion.
- The CLI reads JSON contracts; humans read Markdown specs.
- Example: `docs/kernel-lifecycle.md` + `metadata/kernel-lifecycle.json`.

### 6. Read-Only Core
- `constitution/`, `kernel/`, and `runtime/` are immutable to plugins.
- Only official AIOS updates may modify core layers.

### 7. Zero Hardcoded AI
- AIOS depends on capabilities, not specific LLM providers.
- Model integration uses LLM-Adapter pattern: OpenAI, Anthropic, Gemini, Ollama, LM Corp, etc.

### 8. Single Source of Truth
- version → manifest
- dependencies → dependency registry
- capabilities → capability registry
- rules → rule registry
- No duplicated definitions.

### 9. Backward Compatibility Forever
- Once a public contract is released, it is never silently broken.
- Deprecation must precede removal.
- Removal only in major version increments.

### 10. Extensibility Without Forking
- Organizations extend through plugins, packs, and configuration.
- Never through modifying the core.

### 11. Test Before Release
- Every module includes self-tests, integration tests, and validation rules.
- No production subsystem ships without tests.

### 12. Never Guess Unknown Information
- When AIOS cannot proceed, it:
  1. Explains what is missing
  2. Suggests options
  3. Requests minimal additional input
- It never fabricates unsupported facts.

### 13. Explain Every Decision
- Major architectural recommendations carry:
  - Why it was chosen
  - Alternatives considered
  - Trade-offs
  - Risks
  - Long-term impact

### 14. Trace Everything
```
Feature
  ↓
Workflow
  ↓
Prompt
  ↓
Template
  ↓
Rule
  ↓
Generated Output
```
Every output is traceable to the features, rules, and knowledge that produced it.

### 15. Quality over Quantity
- No empty directories.
- No placeholder files.
- No `// TODO` in shipped modules.
- Every directory contains meaningful, validated assets.

---

## IV. Supremacy

This Constitution governs all AIOS behavior. In case of conflict:
1. The Golden Rule ("correctness over speed")
2. This Constitution
3. Module-level specifications
4. Defaults

No engine, agent, or plugin may override these principles.