# Knowledge Graph Query — TF-IDF Algorithm

## Purpose
Rank relevant nodes when a user queries the knowledge graph with natural language.

## Algorithm
```
1. Tokenize query: lowercase, remove stop words, stem remaining terms
2. For each node in graph:
   a. Build document: node.id + node.type + node.properties (joined)
   b. Compute TF (term frequency): count(term_in_doc) / total_terms_in_doc
   c. Compute IDF (inverse document frequency): log(total_nodes / nodes_containing_term)
   d. TF-IDF score = sum(TF * IDF for each query term)
3. Return top N nodes by TF-IDF score
4. Boost: feature nodes +0.2, symbol nodes +0.1, file nodes base
```

## Performance
- 10k nodes, 5 query terms: ~50ms
- 100k nodes, 5 query terms: ~200ms
- Index pre-built for IDF values, rebuilt on graph version change