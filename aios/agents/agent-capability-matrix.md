# Agent System — Capability Mappings

## Full Agent Capability Matrix

| Agent | Capabilities | Tools | Max Steps | Model |
|-------|-------------|-------|-----------|-------|
| Master | request_routing, capability_routing, execution_lifecycle | read, grep, agent | 10 | flash |
| Architect | design_patterns, system_design | read, grep, agent | 8 | pro |
| Backend | project_context, code_generation | read, write, exec, grep | 15 | pro |
| Frontend | project_context, code | read, write, grep | 12 | flash |
| Database | schema_design | read, write, exec | 8 | flash |
| API | api_design | read, write, grep | 10 | flash |
| QA | framework_analysis, project_tests, test_patterns | read, exec, tests | 10 | flash |
| DevOps | ci_cd_pipeline, containers | read, exec | 8 | flash |
| Security | threat_modeling, audit, compliance | read, grep | 12 | pro |
| Docs | read_file, write_file | read, write | 6 | flash |
| Android | similar to backend | read, write, exec | 10 | flash |
| iOS | similar to backend | read, write, exec | 10 | flash |
| Flutter | similar to backend | read, write, exec | 10 | flash |
| Laravel | similar to backend | read, write, exec | 10 | flash |
| React | similar to frontend | read, write, grep, exec | 12 | flash |
| Reviewer | code_review, sentinel | read, grep | 10 | pro |