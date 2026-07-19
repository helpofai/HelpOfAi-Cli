# AIOS Deployment Guide

## Prerequisites
- HelpOfAi CLI v0.8.89+
- AIOS workspace at project root

## Quick Start

```bash
# 1. Initialize AIOS in your project
hoa module list                    # verify modules are installed
hoa module info AIOS-MODULE-000002 # kernel should be installed

# 2. Run your first workflow
hoa build feature "add health check endpoint"

# 3. Check project health
hoa health
```

## Configuration

AIOS reads from `aios/` directory at the project root. The registry files
(`modules.json`, `capabilities.json`, `dependencies.json`) define what's
available. See `aios/registry/` for the full manifest.

## Profiles

```
hoa profile list          → see available profiles
hoa profile use minimal   → load only kernel + runtime
hoa profile use default   → load all modules
```

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| `module not found` | Module not in registry | Run `hoa registry refresh` |
| `capability unavailable` | Module not loaded | Check profile or dependencies |
| `BRAIN_NOT_INDEXED` | First run | Run a file_indexing workflow first |