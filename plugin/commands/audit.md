---
name: audit
description: Run quality audit (pre-commit checklist or stale doc scan)
arguments:
  - name: mode
    description: "stale" to scan for stale docs, empty for pre-commit checklist
    required: false
---

Run quality audits on the current workspace.

## Usage

```
/audit           # Run pre-commit quality gate (5-question checklist)
/audit stale     # Scan for stale companion docs (>7 days old)
```

## Implementation

For pre-commit quality gate:
```bash
dstack audit --pre-commit
```

For stale doc scan:
```bash
dstack audit --stale
```

## The 5 Questions

1. Did I write negative tests?
2. Did I verify against the LIVE system?
3. Did I update the companion doc with DETAILS?
4. Would these tests FAIL without my code change?
5. Am I moving on because this is TRULY done?

If any answer is "no", stop and finish the work before committing.
