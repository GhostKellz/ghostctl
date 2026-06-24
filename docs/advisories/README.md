# Advisories

This directory records GhostCTL hotfix notes, accepted risks, and resolved
advisories. It is intentionally lightweight: release-critical evidence belongs
here when it helps operators understand why a hotfix shipped and what was
validated.

## Advisory Flow

```mermaid
flowchart TD
    signal["Dependabot, audit finding,\nCI breakage, or user report"] --> triage["triage impact"]
    triage --> decision{"release blocker?"}
    decision -->|yes| hotfix["hotfix notes\nversioned advisory page"]
    decision -->|no| accepted["accepted.md\ntracked risk or watch item"]
    hotfix --> fix["code, dependency,\nworkflow, or docs fix"]
    fix --> verify["fmt, check, test,\nclippy, build, audit"]
    verify --> resolved["resolved.md"]
    accepted --> review["revisit during dependency\nor release review"]
```

## Current Pages

| Page | Purpose |
|------|---------|
| [v0.12.1-hotfix-notes.md](v0.12.1-hotfix-notes.md) | Dependency and workflow hotfix notes for v0.12.1 |
| [accepted.md](accepted.md) | Accepted risks and operational watch items |
| [resolved.md](resolved.md) | Resolved advisory and hotfix history |

## Policy

- Keep advisory notes factual and tied to evidence.
- Prefer exact versions, dates, command names, and validation results.
- Avoid duplicating the full changelog; link back to hotfix notes when needed.
- Record operational caveats separately from code defects.
