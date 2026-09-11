# Workflow Specification: web-debug (AIOS-WORKFLOW-000011)

## Overview
Automated end-to-end debugging workflow for local network websites (`localhost`, `127.0.0.1`, LAN IPs) and web applications.

## Lifecycle Phases
1. **inspect** (`AIOS-MODULE-000024`): Launches headless browser (CDP/Chrome/Edge/Node) and captures runtime exceptions, console errors, and network failures.
2. **correlate** (`AIOS-MODULE-000002`): Locates exact matching source files in local workspace and extracts error line context.
3. **fix** (`AIOS-MODULE-000009`): Generates and applies surgical source code patch.
4. **verify** (`AIOS-MODULE-000010`): Re-inspects the web page to confirm zero remaining errors.
