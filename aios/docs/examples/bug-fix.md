# Example: Bug Fix with Rollback

## Request
```
hoa fix bug "Users can't login with Google OAuth"
```

## Root Cause
OAuth callback URL changed from `/auth/google/callback` to `/auth/social/google/callback`
but the OAuth provider config wasn't updated.

## Fix
- Updated `.env` OAuth redirect URL
- Added validation that redirect URLs match provider config
- Added debug logging for OAuth callback flow

## Rollback
```
hoa rollback                → reverts config change
Old config saved as .env.bak
```

## Verification
```
Tests: 3 new tests added, 12/12 passed
Review: approved (critical=0)
```