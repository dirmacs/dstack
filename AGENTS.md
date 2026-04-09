# dstack — Agent Guidelines

## What This Project Is

dstack is a Rust CLI + Claude Code plugin providing persistent memory, multi-repo sync, VPS deployment, quality gates, and skills management. Published on crates.io as `dstack-memory` and `dstack-cli`.

## For Agents Working On This Codebase

### Before Making Changes

1. Run `cargo test --workspace` — 31+ tests must pass
2. Run `cargo clippy --workspace` — no new warnings
3. Check `dstack audit --pre-commit` — answer all 5 questions

### Key Design Decisions

- **MemoryProvider** is an async trait with File and Eruka backends
- **Config** loads from `~/.config/dstack/config.toml` with `.env` auto-loading
- **No hardcoded paths** — all paths come from config (repos.root, skills_repo, env_file)
- **Plugin format** follows Claude Code spec: hooks.json with SessionStart/PreToolUse/PostToolUse
- **Zola** for static site, not marmite

### What NOT To Do

- Don't add `/opt/` paths anywhere in code or docs
- Don't use `any` types in TypeScript
- Don't commit without running the quality gate
- Don't put secrets in tracked files — use `.env`
- Don't reference internal/client project names (like ehb) in public docs

### Subagent Patterns

When dispatching subagents for dstack work:
- Research agents: read code, grep patterns, check tests
- Implementation: always done in main context, not subagents
- Test after every change: `cargo test --workspace`
- Commit with: `git -c user.name="bkataru" -c user.email="baalateja.k@gmail.com" commit`
