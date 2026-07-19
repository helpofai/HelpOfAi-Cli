# Dependency Brain — Example Dependency Graph

```
User Authentication (FEATURE-000042)
    │
    ├── AuthController.ts
    │   └── calls → AuthService
    │       └── calls → User model
    │
    ├── AuthService.ts
    │   ├── imports → User model
    │   ├── imports → JWT library
    │   └── imports → bcrypt
    │
    └── tests/AuthTest.ts
        └── imports → AuthController

API Rate Limiting (FEATURE-000043)
    │
    ├── depends_on → User Authentication (FEATURE-000042)
    └── RateLimiter.ts
        └── imports → Redis client

Email Notifications (FEATURE-000044)
    │
    ├── depends_on → User Authentication (FEATURE-000042)
    ├── EmailService.ts
    │   └── imports → Mail transport
    └── SendEmailJob.ts
        └── imports → EmailService
```

## Cycle Detection
No cycles detected. Graph is acyclic. ✅

## Orphan Detection
- `src/legacy/OldAuth.ts` — no imports, no dependents → orphan candidate