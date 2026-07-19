# Example: API Migration

## Request
```
hoa build feature "Migrate from REST to GraphQL for /api/v2/users"
```

## Plan
1. Schema design (types, queries, mutations)
2. Resolver implementation
3. Auth middleware conversion
4. Documentation update
5. Deprecate v1 endpoints

## Files Created/Modified
- `graphql/schema.graphql`
- `graphql/resolvers/UserResolver.php`
- `app/Middleware/GraphqlAuth.php`
- `routes/api.php` (modified)
- `docs/api/users.md` (modified)

## Gate Results
- ✅ Architecture: clear migration strategy
- ✅ Tests: 18 passed, 0 failed
- ⚠️ Performance: response time increased 15% (acceptable)
- ✅ Review approved with note: "monitor v1 sunset timeline"

## Rollback Plan
```
hoa rollback                → reverts to REST API
Migration scripts included  → restore v1 routes
```