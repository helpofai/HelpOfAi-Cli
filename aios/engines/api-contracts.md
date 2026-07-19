# Engines — API Contract Reference

## Analysis Engine
```
POST /analyze/architecture
{ "module_id": "AIOS-MODULE-000002" }
→ { "architecture_score": 72, "risks": [...], "recommendations": [...] }

POST /analyze/security
{ "target": "src/auth/" }
→ { "security_score": 85, "findings": [...] }
```

## Code Engine
```
POST /code/generate
{ "plan_step": {...}, "feature_spec": {...} }
→ { "changes": [...] }

POST /code/preview
{ "plan_step": {...} }
→ { "preview": {...}, "estimated_impact": {...} }
```

## Testing Engine
```
POST /test/run
{ "changed_files": ["src/auth.ts"], "project_root": "/ws" }
→ { "total": 24, "passed": 24, "failed": 0, "coverage_delta": 2.3 }
```

## Review Engine
```
POST /review
{ "changes": [...], "profile": "thorough" }
→ { "score": 87, "comments": [...], "pass": true }
```

## Bug Engine
```
POST /bug/analyze
{ "evidence": "TypeError at file.ts:42" }
→ { "root_causes": [...], "severity": "high" }

POST /bug/fix
{ "analysis": {...} }
→ { "fix": {...}, "rollback_ref": "..." }
```

## Security Engine
```
POST /security/scan
{ "manifest_path": "package.json" }
→ { "vulns": [...], "critical_count": 0 }

POST /security/compliance
{ "project_root": "/ws" }
→ { "overall_compliant": true }
```

## DevOps Engine
```
POST /devops/plan
{ "execution_plan": {...}, "environments": ["staging"] }
→ { "deploy_steps": [...], "rollback_steps": [...] }

POST /devops/pipeline
{ "deployment_plan": {...}, "platform": "github_actions" }
→ { "files": [...], "secrets_required": [...] }
```

## Performance Engine
```
POST /perf/profile
{ "target": "src/api/users.ts" }
→ { "bottlenecks": [...], "score": 85 }
```