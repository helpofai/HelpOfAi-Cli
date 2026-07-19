# Engineering Timeline — Sample Data Generator

Run this to generate realistic timeline data for testing:

```json
// aios/.cache/enterprise/gen-timeline.js
const events = [];
const types = ["request", "decision", "phase_start", "phase_complete", "rollback", "release"];
const modules = ["AIOS-MODULE-000002", "AIOS-MODULE-000004", "AIOS-MODULE-000009", "AIOS-MODULE-000010", "AIOS-MODULE-000011"];

for (let i = 0; i < 100; i++) {
  events.push({
    timestamp: new Date(Date.now() - i * 3600000).toISOString(),
    type: types[Math.floor(Math.random() * types.length)],
    summary: `Event ${i} - ${types[Math.floor(Math.random() * types.length)]}`,
    module_id: modules[Math.floor(Math.random() * modules.length)],
    duration_ms: Math.floor(Math.random() * 60000)
  });
}
```

## Sample Query Results
```
hoa timeline --since 2026-07-01 --type rollback
  → 3 rollbacks found:
    1. Migration rollback (5 min) - 2026-07-17 09:45
    2. Config rollback (2 min) - 2026-07-16 14:30
    3. Feature rollback (12 min) - 2026-07-15 11:00
```