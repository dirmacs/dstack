# dstack

Development stack for AI-assisted multi-repo work. Rust CLI + Claude Code plugin.

## Build

```bash
cargo build --release
cargo test --workspace
```

## Install

```bash
cp target/release/dstack /usr/local/bin/
```

## Configuration

Config at `~/.config/dstack/config.toml`. Secrets at `~/.config/dstack/.env` (auto-loaded).

## CLI Commands

```bash
dstack config                        # show configuration
dstack memory load/save/query/export # persistent context (File or Eruka backend)
dstack sync --status                 # cross-repo git status with ahead/behind
dstack deploy <service>              # build → restart → smoke test
dstack deploy <service> --rollback   # rollback to previous binary
dstack skills list/install/sync      # manage skills from skills repo
dstack audit                         # workspace summary
dstack audit --pre-commit            # 5-question quality gate
dstack audit --stale                 # find stale companion docs
```

## Architecture

- `crates/dstack-memory/` — MemoryProvider trait, FileProvider (JSON), ErukaProvider (REST)
- `crates/dstack-cli/` — CLI binary + library (clap, tokio, serde)
- `plugin/` — Claude Code plugin (6 skills, 3 hooks, 3 commands)
- `site/` — Zola static site (dirmacs.github.io/dstack)

## Conventions

- Git author: `bkataru <baalateja.k@gmail.com>`
- No hardcoded `/opt` paths — use config values
- No `any` types in TypeScript
- Run `dstack audit --pre-commit` before every commit
- Eruka search is POST `/api/v1/context/search` with JSON body `{"query": "..."}`
