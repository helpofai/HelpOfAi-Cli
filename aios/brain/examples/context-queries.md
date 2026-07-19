# Context Brain — Query Examples

## Example 1: Build Auth Feature
```
INPUT: "Add email/password authentication with JWT"
OUTPUT: {
  relevant_files: ["src/auth/AuthController.ts", "src/auth/AuthService.ts", "src/models/User.ts"],
  relevant_features: ["FEATURE-000042 (User Authentication)"],
  relevant_deps: {"src/auth/AuthService.ts": ["src/models/User.ts"]},
  project_summary: "Laravel + TypeScript project with existing auth base",
  estimated_tokens: 1200
}
```

## Example 2: Fix Bug
```
INPUT: "Fix login page crash on invalid token"
OUTPUT: {
  relevant_files: ["src/auth/AuthController.ts:15-30", "src/auth/AuthService.ts:40-50"],
  relevant_features: ["FEATURE-000042"],
  relevant_deps: {},
  project_summary: "Error in AuthService.validateToken when token format is malformed",
  estimated_tokens: 800
}
```

## Example 3: Refactor
```
INPUT: "Extract auth logic into separate module"
OUTPUT: {
  relevant_files: ["src/auth/AuthController.ts", "src/auth/AuthService.ts", "src/middleware/AuthMiddleware.ts"],
  relevant_features: ["FEATURE-000042"],
  relevant_deps: {"AuthController": ["AuthService", "AuthMiddleware"]},
  project_summary: "Auth logic spread across 3 files — candidate for extraction",
  estimated_tokens: 1500
}
```