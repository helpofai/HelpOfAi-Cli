# Change Risk Prediction — Regression Model

## Purpose
Predict the risk level of a proposed code change before implementation, based on historical patterns.

## Features
| Feature | Weight | Description |
|---------|--------|-------------|
| files_changed | 0.15 | Number of files in the change |
| dependencies_affected | 0.20 | Number of downstream dependents |
| lines_of_code | 0.10 | Total LOC changed |
| previous_bugs | 0.25 | Bug count in the target files (last 90 days) |
| author_experience | 0.10 | How many times the author has changed these files |
| test_coverage | 0.15 | Test coverage of the target files |
| time_since_last_change | 0.05 | Days since the files were last modified |

## Risk Score
```
risk = sum(feature_i * weight_i)
normalized to 0.0 - 1.0

risk < 0.3: LOW — safe to proceed
risk 0.3-0.6: MEDIUM — require review
risk > 0.6: HIGH — require review + rollback plan
```

## Training
The model is trained on historical execution data from the History Brain.
It improves over time as more changes are recorded.
Initial weights are set from heuristic defaults.