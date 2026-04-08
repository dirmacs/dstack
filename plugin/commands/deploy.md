---
name: deploy
description: Deploy a service (build + restart + smoke test)
arguments:
  - name: service
    description: Service name from config, or "all"
    required: false
---

Deploy a service using the dstack deployment pipeline.

## Usage

```
/deploy ares        # Deploy a specific service
/deploy all         # Deploy all configured services
/deploy             # Show available deploy targets
```

## What It Does

1. Runs the configured build command
2. Restarts the systemd service
3. Runs the smoke test (if configured)

## Implementation

If a service name is provided, run:
```bash
dstack deploy {{service}}
```

If "all" is provided or `--all` flag:
```bash
dstack deploy --all
```

If no argument, show available targets:
```bash
dstack config | grep -A 20 "Deploy targets"
```

Before deploying, verify disk space and that tests pass.
