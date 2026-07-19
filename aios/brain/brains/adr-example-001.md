# ADR-000001: AIOS Architecture Pattern

**Status:** Accepted  
**Context:** AIOS needed a modular architecture where each component is independently loadable.
**Alternatives considered:** Monolithic (rejected — too rigid), Microkernel (rejected — too complex for v1)
**Decision:** Use Clean Architecture with Event-Driven transitions between layers. Each module declares capabilities and dependencies in its manifest. The kernel routes requests by capability match.
**Consequences:** + modular, + testable, + independently deployable — but requires stricter contract management.
**Recorded by:** AIOS v1.0 Constitution