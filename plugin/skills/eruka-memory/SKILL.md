---
name: eruka-memory
description: Guide for persisting learnings, decisions, and patterns to dstack memory
---

# Eruka Memory

Persist non-obvious learnings so future sessions start with context instead of rediscovering it.

## When to Save

Save when you discover something that:
- Would be lost when this conversation ends
- A future session would waste time rediscovering
- Corrects an assumption or reveals a non-obvious pattern

## What to Save

| Save | Don't Save |
|------|------------|
| Architectural decisions and WHY | Code that's in git |
| Bug root causes | Ephemeral task state |
| Non-obvious API behaviors | Things derivable from `git log` |
| Build/deploy gotchas | Temporary workarounds |
| User corrections and preferences | Duplicate of existing memory |

## How to Save

```bash
# Key format: category/project/topic
dstack memory save "learnings/ares/jwt-fix" "jsonwebtoken needs rust_crypto feature, not default"
dstack memory save "decisions/eruka/field-rollback" "Fields are never deleted, only set to confidence=0"
dstack memory save "gotchas/deploy/ares-build" "Must use lto=thin, full LTO OOMs on 2GB VPS"
```

## Memory Hygiene

1. Before saving, run `dstack memory query <keyword>` to check for duplicates
2. Use descriptive keys — future you searches by keyword
3. Include WHY, not just WHAT
4. Periodically review with `dstack memory export` and prune stale entries

## Backends

- **file** (default): JSON files at `~/.local/share/dstack/memory/`
- **eruka**: REST API to Eruka context engine (set `$DSTACK_ERUKA_KEY`)

Switch backends in `~/.config/dstack/config.toml` under `[memory]`.
