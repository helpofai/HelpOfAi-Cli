# Roadmap Generator — Example Output

## Project: E-commerce Platform v2

### Phases

| Phase | Features | Start | End | Risks |
|-------|----------|-------|-----|-------|
| Q3 Foundation | Auth, Products, Cart | 2026-07-01 | 2026-08-15 | None |
| Q4 Payments | Checkout, Payments, Orders | 2026-08-16 | 2026-10-30 | ⚠️ Payment gateway API unknown |
| Q1 Expansion | Reviews, Search, Recommendations | 2026-11-01 | 2027-01-15 | ⚠️ Search depends on Reviews |

### Critical Path
```
Auth → Products → Cart → Checkout → Payments → Orders
(any delay on these features delays the entire project)
```

### Total Duration Estimate
- Optimistic: 4.5 months
- Expected: 6 months
- Pessimistic: 8 months (if payment integration hits delays)