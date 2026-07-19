# AIOS CLI Commands Reference

## Workflow Commands
```
hoa build feature "<description>"    → run build-feature workflow
hoa fix bug "<description>"          → run fix-bug workflow
hoa review [path]                    → run review-code workflow
hoa refactor [path]                  → run refactor workflow
hoa upgrade [package]                → run upgrade workflow
hoa optimize                         → run optimize workflow
hoa analyze|audit                    → run analyze workflow
hoa release                          → run release workflow
hoa rollback                         → revert last change
```

## Module Commands
```
hoa module list                     → list installed modules
hoa module info <id>                → show module details
hoa capability list                 → list all capabilities
hoa capability resolve <cap>        → find providing module
```

## Brain Commands
```
hoa brain index                     → run file_indexing
hoa brain query "<question>"        → query knowledge graph
hoa brain status                    → check staleness
```

## Enterprise Commands
```
hoa simulate <plan_id>              → run digital twin simulation
hoa timeline                        → view engineering timeline
hoa decision list                   → list decisions
hoa decision search "<query>"       → search decisions
hoa health                          → project health dashboard
hoa knowledge search "<query>"       → search knowledge base
hoa roadmap --features F1,F2        → generate roadmap
```

## Profile Commands
```
hoa profile list                    → list profiles
hoa profile use <name>              → switch profile
hoa profile create <name>           → create profile
```

## Utility Commands
```
hoa help                            → this help
hoa version                         → show version
hoa status                          → show system status
hoa cache clear                     → clear all caches
```