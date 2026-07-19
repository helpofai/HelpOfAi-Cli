# Roadmap Generator — Capacity Models

## Model: Simple
```
velocity = 5 features/week/engineer
total_weeks = total_features / (engineers * velocity)
```
Use when features are well-understood and no external blockers.

## Model: Conservative
```
velocity = 3 features/week/engineer
buffer = total_weeks * 0.2
total_weeks = (total_features / (engineers * velocity)) + buffer
```
Use when dependencies are uncertain or features have high complexity.

## Model: Aggressive
```
velocity = 7 features/week/engineer
parallel_factor = 1.3 (if features can be worked on independently)
total_weeks = total_features / (engineers * velocity * parallel_factor)
```
Use for well-understood projects with experienced team. Flag as high confidence.

## Capacity Override
```
hoa roadmap --features F1..F10 --capacity 3 --velocity conservative
→ 10 features, 3 engineers, conservative model = 4.8 weeks
```