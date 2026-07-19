# Multi-Feature Dependency Chain Example

```
Feature A: User Authentication (implemented)
  Depends on: none
  Files: AuthController.ts, AuthService.ts, User.ts
  Impact scope: high (used by 4 other features)

Feature B: Shopping Cart (implemented)
  Depends on: Feature A
  Files: CartController.ts, CartService.ts
  Impact scope: medium

Feature C: Checkout (in progress)
  Depends on: Feature A, Feature B
  Files: CheckoutController.ts, PaymentService.ts
  Impact scope: high (handles payments)

Feature D: Order History (planned)
  Depends on: Feature C
  Files: OrderController.ts
  Impact scope: low

Change: Modify Feature A (User model changes)
  Direct impact: [B, C]
  Indirect impact: [D]
  Cascading impact: []
  Total files affected: 7
  Risk: HIGH (4 dependents, payment flow affected)
  Recommendation: Stage changes — update A, verify B+C, then D
```