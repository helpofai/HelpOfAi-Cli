# Security Engine — Vulnerability Severity Scoring

## CVSS-Based Scoring
```
severity = based on CVSS 3.1:
  CRITICAL: CVSS >= 9.0
  HIGH: CVSS >= 7.0
  MEDIUM: CVSS >= 4.0
  LOW: CVSS < 4.0
```

## Custom Severity for Code Findings
```
severity = (exploitability * 0.4) + (impact * 0.4) + (scope * 0.2)

exploitability: 0-10 (how easy to exploit)
impact: 0-10 (damage if exploited)
scope: 0-10 (blast radius, affected components)
```

## Compliance Severity
```
BLOCKER: violates regulatory requirement (GDPR, SOC2, HIPAA)
HIGH: violates security policy
MEDIUM: violates best practice
LOW: informational
```