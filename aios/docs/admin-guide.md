# AIOS Enterprise Administration Guide

## System Monitoring

Monitor AIOS health and performance:

```
hoa health                    → dashboard
hoa health --verbose          → detailed metrics
hoa timeline                  → engineering activity log
hoa timeline --since 1 month  → compliance audit
```

## Module Management

```
hoa module list               → all installed modules
hoa module info kernel        → kernel details
hoa capability list           → all capabilities
```

## Profile Management

```
hoa profile list              → see available profiles
hoa profile use minimal       → minimal load for simple tasks
hoa profile use audit         → security audit mode
hoa profile create custom     → create custom profile
```

## Cache Management

```
hoa cache clear               → clear all caches
hoa cache clear --brain       → clear brain cache only
hoa cache clear --runtime     → clear runtime cache only
```

## Recovery

If AIOS enters an inconsistent state:

```
hoa status                    → check system state
hoa repair                    → attempt automatic repair
hoa repair --force            → force re-index everything
```

## Backup

The following directories should be backed up:
- `aios/registry/` — module registry (infrequently changed)
- `aios/.cache/brain/` — brain data (frequently changed)
- `aios/.cache/enterprise/` — enterprise configuration

## Security

- Plugins run sandboxed per their declared permissions
- Core modules are immutable to plugins (Engineering Law 16)
- All decisions are logged (Constitution Principle 14)
- Audit trail available via `hoa timeline --export audit.md`