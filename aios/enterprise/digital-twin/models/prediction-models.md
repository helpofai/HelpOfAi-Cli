# Digital Twin — Prediction Models

## Model: File Impact Prediction
Estimates which files a feature request will touch based on similarity to
previously indexed features.

```
candidates = index.search(feature_description, top_k=10)
predicted_files = weighted_sample(candidates.files, candidates.similarity)
confidence = mean(candidates.similarity)
```

Accuracy improves as the brain indexes more features.

## Model: Effort Estimation
```
effort_minutes = (files_predicted * 5) + (new_modules * 15) + (external_apis * 30)
confidence_adjustment = 0.3 if no similar features indexed else 0.1
```

## Model: Conflict Prediction
```
probability = (active_plans_on_path / total_active_plans) * (file_overlap_ratio)
if probability > 0.5: FLAG
```

## Training Data
Stored in `aios/.cache/enterprise/digital-twin/`. Updated after each
completed feature with actual vs predicted values for self-improvement.