---
name: sync
description: Git sync across tracked repos
arguments:
  - name: mode
    description: "status" to show status, "dry-run" for dry run, empty to sync
    required: false
---

Sync git state across all tracked repositories.

## Usage

```
/sync            # Pull + push all clean repos
/sync status     # Show branch and sync state for all repos
/sync dry-run    # Show what would happen without pushing
```

## Implementation

For status:
```bash
dstack sync --status
```

For dry run:
```bash
dstack sync --dry-run
```

For full sync:
```bash
dstack sync
```

Only repos with clean working trees are synced. Dirty repos are skipped with a warning.
