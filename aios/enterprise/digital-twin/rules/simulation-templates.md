# Digital Twin — Simulation Templates

## Template: Feature Addition
```
{
  "type": "create",
  "files": ["controller", "service", "model", "migration", "test", "route"],
  "pattern": "add_feature"
}
```

## Template: Bug Fix
```
{
  "type": "modify",
  "files": ["existing + test"],
  "pattern": "fix_bug"
}
```

## Template: Refactor
```
{
  "type": "modify",
  "files": ["existing"],
  "pattern": "refactor",
  "risk": "high"
}
```

## Template: Migration
```
{
  "type": "create + modify",
  "files": ["migration", "model", "seeder"],
  "pattern": "database_change",
  "rollback_always": true
}
```