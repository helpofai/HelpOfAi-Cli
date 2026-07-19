# Integration — Performance Budgets

## Startup
| Operation | Max Time | Notes |
|-----------|----------|-------|
| Full load (28 modules) | 500ms | Parallelized |
| Minimal load (3 modules) | 100ms | |
| Plugin scan | 50ms | Per plugin directory |

## Runtime
| Operation | Max Time | Notes |
|-----------|----------|-------|
| Capability resolution | 10ms | In-memory lookup |
| Workflow builder | 50ms | Intent + template |
| Prompt assembly | 50ms | Template + context injection |
| Registry refresh | 200ms | File read + index rebuild |

## Cache
| Operation | Max Time | Notes |
|-----------|----------|-------|
| Brain cache read | 20ms | If version matches |
| Brain cache write | 100ms | Graph serialization |
| Cache clear | 10ms | File delete