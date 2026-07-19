# Brain Data — ADR Examples (Decision Brain)

## ADR-010: Choose Web Framework
```
Status: Accepted
Date: 2026-07-01
Context: Need to choose a web framework for the new API service.
Alternatives: Express (Node.js), FastAPI (Python), Spring Boot (Java)
Decision: FastAPI — Python is the team's strongest language, async support is built-in.
Consequences: + fast development, + async performance, - team needs Python experience
References: sym://framework_decision, file://docs/adr/adr-010.md
```

## ADR-011: Database Selection
```
Status: Accepted
Date: 2026-07-02
Context: Need primary data store for transactional data.
Alternatives: PostgreSQL, MySQL, MongoDB, SQLite
Decision: PostgreSQL — strongest feature set, ACID compliant, best ecosystem.
Consequences: + reliability, + feature-rich, - operational overhead vs SQLite
References: file://config/database.php, feat://FEATURE-000001
```

## ADR-012: Cache Strategy
```
Status: Proposed
Date: 2026-07-05
Context: API response times degrading under load, need caching layer.
Alternatives: Redis, Memcached, Varnish
Decision: Redis (proposed) — supports data structures needed for rate limiting and sessions.
Consequences: + multi-purpose, + persistence option, - additional infrastructure
References: sym://AuthService.validateToken, sym://RateLimiter
```