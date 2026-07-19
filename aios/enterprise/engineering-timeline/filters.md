# Engineering Timeline — Filter Config

## Date Presets
- `--today` — current day only
- `--this-week` — last 7 days
- `--this-month` — last 30 days
- `--since <date>` — ISO date or relative "2 weeks ago"

## Event Filters
- `--type request` — user requests only
- `--type decision` — ADRs only
- `--type phase_start,phase_complete` — lifecycle events
- `--type rollback` — failure events
- `--type release` — deployment events

## Output Formats
```
hoa timeline --since 2026-07-01 --format json
hoa timeline --type rollback --format markdown
hoa timeline --this-week --format dashboard
```

## Export
```
hoa timeline --since 2026-01-01 --export audit-report.md
```