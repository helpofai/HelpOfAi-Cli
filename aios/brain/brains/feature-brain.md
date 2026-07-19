# Feature Brain — AIOS-BRAIN-000002

Tracks features and their implementation files. When the feature module
(AIOS-MODULE-000008) creates a feature spec, the Feature Brain indexes its
files and tracks coverage.

### Schema
- `features[]`: id, name, files[], depends_on[], status (planned|implemented|tested|released)
- `spec_ref`: links to AIOS-FEATURE-NNNNNN