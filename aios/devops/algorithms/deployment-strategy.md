# DevOps Engine — Deployment Strategy

## Strategy Selection
```
if environment == "production":
    strategy = "blue_green"  # zero downtime
elif environment == "staging":
    strategy = "canary"      # 10% traffic for 15min
elif environment == "dev":
    strategy = "direct"      # immediate replacement
```

## Blue-Green Steps
```
1. Build new version → tag as "green"
2. Spin up green environment
3. Run health checks on green
4. Switch load balancer to green
5. Keep blue running (rollback ready)
6. Monitor for 15min
7. If healthy: terminate blue
8. If unhealthy: switch back to blue
```

## Rollback Triggers
```
- Health check failure → auto-rollback within 30s
- Error rate > 1% → auto-rollback within 60s
- Latency p99 > 2x baseline → auto-rollback within 120s
- Manual "hoa rollback" → immediate
```