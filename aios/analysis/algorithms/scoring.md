# Analysis Engine — Scoring Algorithms

## Architecture Score
```
score = (coupling_score * 0.3) + (cohesion_score * 0.3) + (layer_violations * -0.2) + (circular_deps * -0.2)
normalized to 0-100

coupling_score: lower is better (fewer dependencies between modules)
cohesion_score: higher is better (related code grouped together)
layer_violations: each violation reduces score
circular_deps: each cycle detected reduces score
```

## Security Score
```
score = 100 - (critical * 25) - (high * 10) - (medium * 3)
capped at 0-100

critical: 25 points deducted each
high: 10 points deducted each
medium: 3 points deducted each
```

## Quality Gate
```
pass = architecture >= 60 AND security >= 60 AND no critical findings
```