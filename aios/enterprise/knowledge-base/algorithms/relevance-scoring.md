# Knowledge Base — Relevance Scoring

Search results are ranked by relevance score:

```
score = title_match * 0.4 + content_match * 0.3 + tag_match * 0.15 + recency * 0.1 + source_weight * 0.05
```

Where:
- `title_match`: 1.0 if search terms appear in title, 0.5 partial, 0.0 none
- `content_match`: TF-IDF score normalized 0-1
- `tag_match`: proportion of search terms matching entry tags
- `recency`: 1.0 for <7 days, 0.8 for <30, 0.5 for <90, 0.2 for older
- `source_weight`: operator=1.0, detected=0.7, explicit=0.9

Minimum score for inclusion: 0.15. Top 5 results returned by default.