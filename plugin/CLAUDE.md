# dstack — Development Stack Plugin

This plugin provides persistent memory, multi-repo sync, VPS deployment, and quality gates for AI-assisted development.

## Prerequisites

Install the `dstack` CLI:

```bash
cargo install dstack-cli
```

Or build from source:

```bash
cd /opt/dstack && cargo build --release
cp target/release/dstack /usr/local/bin/
```

## Configuration

Create `~/.config/dstack/config.toml`:

```toml
[memory]
backend = "file"  # or "eruka"

[repos]
tracked = ["/opt/my-repo"]

[deploy.my-service]
build = "cd /opt/my-repo && cargo build --release"
service = "my-service"
smoke = "curl -sf http://localhost:3000/health"
```

## Available Skills

- **using-dstack** — Session orientation, available commands
- **eruka-memory** — When and how to persist learnings
- **multi-repo-ops** — Cross-repo dependency awareness and sync
- **vps-deploy** — Build + restart + smoke test deployment
- **companion-docs** — Track implementation reality vs plan intent
- **quality-gates** — Pre-commit 5-question checklist

## Available Commands

- `/deploy [service]` — Deploy a service or all services
- `/sync` — Sync tracked repos
- `/audit` — Run quality gate checklist
