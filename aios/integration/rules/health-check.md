# Integration — Health Check

## Health Dimensions
| Component | Check | Score |
|-----------|-------|-------|
| Loader | all modules loaded successfully | 0=failed, 100=perfect |
| Registry Reader | all abilities indexed | 0=failed, 100=perfect |
| Workflow Builder | intent classification accuracy | 0=poor, 100=perfect |
| Prompt Assembler | average assembly time | 0=slow, 100=fast |
| Plugin Loader | plugins loaded successfully | 0=failed, 100=perfect |
| Brain Cache | cache hit rate | 0=poor, 100=excellent |

## CLI Output
```
hoa integration health
→ Overall: 93/100 🟢
  Loader:       100/100 🟢 (28/28 modules loaded)
  Registry:     100/100 🟢 (34 capabilities indexed)
  Workflow:      89/100 🟢 (89% intent accuracy)
  Prompt:        95/100 🟢 (avg 45ms assembly)
  Plugin:       100/100 🟢 (0 plugins — nothing to fail)
  Cache:         82/100 🟢 (82% hit rate)
```