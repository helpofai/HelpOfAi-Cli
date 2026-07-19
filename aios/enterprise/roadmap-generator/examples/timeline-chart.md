# Roadmap Generator — Timeline Visualization

Chart output for terminal rendering:

```
Q3                    Q4                    Q1
Jul Aug Sep          Oct Nov Dec          Jan Feb Mar

Auth ████████░░░░░░
Products  ████████░░░░░░
Cart     ████████░░░░░░
Checkout      ██████████░░░░░
Payments       ██████████░░░░░
Orders        ██████████░░░░░
Reviews              ████████████
Search               ████████████
Recommendations       ████████████

     ██ = actual   ░░ = buffer
     ⚠️ = risk flagged
```

## Legend
- `████████` = active development
- `░░░░░░░░` = buffer/contingency
- `⚠️` = risk flagged (external dependency, complex, or unclear scope)
- `|` = phase boundary