# Code Similarity Matching — Locality-Sensitive Hashing

## Purpose
Find similar code across the project to detect duplication, suggest refactoring candidates, and identify related implementations.

## Algorithm
```
1. For each indexed file:
   a. Extract function/class bodies as separate documents
   b. Compute minhash signature (100 hash functions)
   c. Band the signature into bands of size 5
   d. Hash each band into a bucket
2. Candidate pairs: documents that fall into the same bucket
3. Verify candidates with Jaccard similarity:
   similarity = |A ∩ B| / |A ∪ B|
   where A, B are shingle sets (n-gram tokens of the code)
4. Return pairs with similarity > threshold (default: 0.7)
```

## Performance
- 10k functions: ~500ms to index, ~200ms to query
- 100k functions: ~5s to index, ~2s to query
- Threshold adjustment: lower = more candidates, higher = stricter

## Applications
- Duplicate code detection (similarity > 0.9)
- Related implementation discovery (similarity > 0.7)
- Refactoring opportunities (similarity > 0.5)