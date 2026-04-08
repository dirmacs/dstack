---
name: vps-deploy
description: VPS deployment workflow — build, restart, smoke test, rollback guidance
---

# VPS Deploy

Deploy services via `dstack deploy`. Each target defines a build command, systemd service name, and optional smoke test.

## Deploy a Service

```bash
# Single service
dstack deploy ares

# All configured services
dstack deploy --all
```

## Pipeline

1. **Build** — runs the configured build command (e.g., `cargo build --release`)
2. **Restart** — `sudo systemctl restart <service>`
3. **Smoke test** — runs configured smoke command (e.g., `curl -sf http://localhost:3000/health`)

If the smoke test fails, you get a warning. Consider rollback.

## Pre-Deploy Checklist

- [ ] All tests pass (`cargo test --workspace`)
- [ ] Build succeeds in release mode
- [ ] No uncommitted changes in the repo
- [ ] Companion doc updated with deployment details
- [ ] Disk space adequate (`df -h /opt`)

## Rollback

If a deploy goes wrong:

1. Check logs: `sudo journalctl -u <service> -n 50`
2. Revert to last known-good binary (keep a backup)
3. Restart: `sudo systemctl restart <service>`
4. Verify: run the smoke test manually

## Configuration

Deploy targets are defined in `~/.config/dstack/config.toml`:

```toml
[deploy.ares]
build = "cd /opt/ares && cargo build --release"
service = "ares"
smoke = "curl -sf http://localhost:3000/health"

[deploy.eruka]
build = "cd /opt/eruka && cargo build --release"
service = "eruka"
smoke = "curl -sf http://localhost:8081/health"
```

## Disk Hygiene

Before building, check available disk space. Rust release builds consume significant space. Clean old target directories if needed:

```bash
df -h /opt
du -sh /opt/*/target 2>/dev/null
```
