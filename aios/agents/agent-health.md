# Agent System — Health Check

## Agent Health Dimensions
| Dimension | Check | Scoring |
|-----------|-------|---------|
| Dispatch Accuracy | % of tasks dispatched to correct agent | 0=poor, 100=perfect |
| Queue Depth | pending tasks in queue | 0=excellent, 100=severe backlog |
| Fail Rate | % of agent tasks that fail | 0=perfect, 100=poor |

## CLI Output
```
hoa agent health
→ Overall: 92/100 🟢
  Dispatch: 94/100 🟢 (47/50 correct dispatches today)
  Queue:    90/100 🟢 (2 tasks queued)
  Fail:     92/100 🟢 (4 of 50 tasks failed today)
```