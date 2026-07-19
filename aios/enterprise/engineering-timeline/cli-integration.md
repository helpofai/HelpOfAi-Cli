# Engineering Timeline — CLI Integration

The timeline is built on the History Brain (AIOS-BRAIN-000008) and
the memory module's event log (AIOS-CONTRACT-000060).

## Data Flow
```
Every phase transition → event_bus (AIOS-CONTRACT-000024) 
  → logged to memory namespace "events" 
  → timeline reads from memory for queries
```

## Performance
- Default query: last 50 events, milliseconds
- Full day query: ~1,000 events, < 100ms
- Full month query: ~30,000 events, < 2s
- Export to file: large queries written progressively

## Storage
Events are append-only. Retention defaults to 365 days.
Auto-prune is disabled by default (manual archive recommended).