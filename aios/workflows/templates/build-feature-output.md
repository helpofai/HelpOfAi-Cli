# Build Feature — Example Output

## Feature: User Authentication (Email + OAuth)

### Plan
1. Database migration for users table + oauth_identities
2. Email/password auth endpoints (register, login, logout)
3. OAuth provider integration (Google, GitHub)
4. JWT token service
5. Frontend login/signup pages

### Files Created
- `database/migrations/xxxx_create_users_table.php`
- `database/migrations/xxxx_create_oauth_identities_table.php`
- `app/Http/Controllers/AuthController.php`
- `app/Services/JwtService.php`
- `app/Models/User.php`
- `resources/js/pages/Login.tsx`
- `resources/js/pages/Signup.tsx`
- `tests/Feature/AuthTest.php`

### Gate Results
- ✅ architecture review: passed (clean layering)
- ✅ security review: passed (JWT rotation, rate limiting)
- ✅ tests: 24 passed, 0 failed
- ✅ review approved