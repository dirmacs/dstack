---
name: using-dstack
description: Session orientation — explains dstack, shows available commands, confirms memory loaded
---

# Using dstack

dstack is a development stack for AI-assisted multi-repo work. It provides persistent memory, multi-repo sync, VPS deployment, and quality gates.

## Available Commands

| Command | Purpose |
|---------|---------|
| `dstack config` | Show current configuration |
| `dstack memory load` | Load memory fields for current project |
| `dstack memory save <key> <value>` | Persist a learning or decision |
| `dstack memory query <pattern>` | Search memory by keyword |
| `dstack memory export` | Export all memory as JSON |
| `dstack sync --status` | Show git status across tracked repos |
| `dstack sync` | Pull + push all clean tracked repos |
| `dstack deploy <service>` | Build + restart + smoke test a service |
| `dstack deploy --all` | Deploy all configured services |
| `dstack audit --pre-commit` | Run 5-question quality gate |
| `dstack audit --stale` | Scan for stale companion docs |

## On Session Start

1. Memory is automatically loaded via the session-start hook
2. Check `dstack config` to see tracked repos and deploy targets
3. Use `dstack sync --status` to see cross-repo git state

## Compatibility

dstack works alongside other Claude Code plugins:
- **superpowers** handles TDD, debugging, verification workflows
- **GSD** handles project structure, waves, milestones
- **dstack** handles memory, deployment, multi-repo ops, quality gates

No conflicts. Skills use the `dstack:` namespace.
