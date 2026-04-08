---
name: multi-repo-ops
description: Cross-repo dependency awareness, sync protocol, and build order for multi-repo work
---

# Multi-Repo Operations

Use when work touches 2+ repos in the tracked list.

## Quick Status

```bash
dstack sync --status
```

Shows branch, clean/dirty, ahead/behind for every tracked repo.

## Sync Protocol

```bash
# Dry run first
dstack sync --dry-run

# Pull + push all clean repos
dstack sync
```

Only syncs repos with clean working trees. Dirty repos are skipped with a warning.

## Build Order

When changes span repos, build in dependency order:

1. **Libraries first** — shared crates, core types
2. **Servers next** — API servers, workers, daemons
3. **Clients last** — CLI tools, frontends, config

Example order:
```
core-lib → api-server → client-app
memory-engine → mcp-bridge
cli-tool (standalone)
```

## Cross-Repo Checklist

Before committing cross-repo changes:

- [ ] All affected repos compile independently
- [ ] No version mismatches in shared dependencies
- [ ] Tests pass in each repo separately
- [ ] Branch names align (holy/holy or main/main)
- [ ] Sync status is clean after pushing all repos

## Branch Alignment

Some projects use feature branches or non-default branches. Check `dstack sync --status` to see which branch each repo is on. Don't mix — if repo A is on a feature branch, keep it there.

## Configuration

Tracked repos are set in `~/.config/dstack/config.toml`:

```toml
[repos]
tracked = ["my-api", "my-lib", "my-frontend"]
```
