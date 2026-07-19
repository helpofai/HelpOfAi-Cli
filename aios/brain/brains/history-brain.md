# History Brain — AIOS-BRAIN-000008

Append-only engineering timeline. Every request, decision, and outcome is a
timestamped event. Used for audit trails, engineering metrics, and the project
health dashboard.

### Event types
- `request`: user request received
- `decision`: architectural decision recorded
- `phase_start` / `phase_complete`: lifecycle phase transitions
- `rollback`: a rollback was triggered
- `release`: a release was created

### Storage
- Events are persisted via memory module (AIOS-CONTRACT-000060)
- Append-only: events are never modified or deleted