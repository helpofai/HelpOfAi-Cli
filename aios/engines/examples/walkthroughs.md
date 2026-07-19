# Engines — Example Walkthroughs

## Example: Full Feature Build (Analysis + Code + Test + Review)
```
1. hoa analyze architecture
   → Architecture score: 72 (needs improvement)

2. hoa code generate --plan "add auth feature"
   → Created: AuthController.ts, AuthService.ts, User.ts

3. hoa test run
   → 24/24 passed, coverage +2.3%

4. hoa review
   → 0 critical, 2 minor (style) → pass ✓
```

## Example: Bug Fix
```
1. hoa bug analyze "TypeError at AuthService.ts:42"
   → Root cause: null email field, confidence 0.85

2. hoa bug fix <analysis_id>
   → Patch generated with rollback

3. hoa test run
   → 3 new tests added, all passed

4. hoa review
   → 0 critical → pass ✓
```

## Example: Security Audit
```
1. hoa security scan package.json
   → 1 high CVE, 2 medium

2. hoa security compliance
   → SOC2 ready ✓, GDPR gaps found

3. hoa security audit
   → Full report with remediation steps
```