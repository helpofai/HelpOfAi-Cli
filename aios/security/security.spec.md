# AIOS Security Platform — AIOS-MODULE-000012

> **Codename:** Sentinel · **Version:** 1.0.0 · **Layer:** 5 (engines)
> **Machine contract:** [`module.json`](./module.json) (authoritative)

## 1. Purpose

Vulnerability scanning, CVE cross-referencing, compliance checking, and security
audit for AIOS projects. Offline-first with cached CVE database.

## 2. Contracts

| ID | Capability |
|----|------------|
| `AIOS-CONTRACT-000092` | `vulnerability_scanning` |
| `AIOS-CONTRACT-000093` | `compliance_checking` |

## 3. Dependencies

Requires: kernel, brain. Optional: analysis. Load order: 23 (optional).