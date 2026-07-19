# Example: Build Auth Feature

## Request
```
hoa build feature "Add email/password authentication with JWT"
```

## Generated Plan
1. User migration (email, password_hash, email_verified_at)
2. Auth controller (register, login, logout, refresh)
3. JWT service (generate, validate, rotate)
4. Middleware (authenticate, optional)
5. Tests (register, login, invalid, refresh, protected routes)

## Files Created
- `migrations/xxxx_create_users_table.php`
- `app/Models/User.php`
- `app/Http/Controllers/AuthController.php`
- `app/Services/JwtService.php`
- `app/Middleware/AuthenticateJwt.php`
- `tests/Feature/AuthTest.php`

## Gate Results
- ✅ Architecture review: clean layering
- ✅ Security review: bcrypt, JWT rotation, rate limiting
- ✅ Tests: 24 passed, 0 failed
- ✅ Review approved

## Execution Report
```
Workflow: build-feature
Phases: 6/6 completed
Duration: 12 min
Files: 8 created, 1 modified
```