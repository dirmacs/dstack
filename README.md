# dstack

Development stack for AI-assisted multi-repo work. Persistent memory, quality gates, VPS deployment, cross-repo sync.

**Rust CLI + Claude Code plugin** that encodes battle-tested workflows from production work across 15+ repos.

## Install

```bash
cargo install dstack-cli
```

Or from source:

```bash
git clone https://github.com/dirmacs/dstack
cd dstack
cargo build --release
cp target/release/dstack /usr/local/bin/
```

## Configure

Create `~/.config/dstack/config.toml`:

```toml
[memory]
backend = "file"  # "file" (default) or "eruka"

[repos]
tracked = ["/opt/my-project", "/opt/my-lib"]

[deploy.my-service]
build = "cd /opt/my-project && cargo build --release"
service = "my-service"
smoke = "curl -sf http://localhost:3000/health"
```

## CLI Usage

```bash
dstack config                              # Show configuration
dstack memory load --project myapp         # Load project memory
dstack memory save "key" "value"           # Persist a learning
dstack memory query "pattern"              # Search memory
dstack sync --status                       # Cross-repo git status
dstack sync                                # Pull + push clean repos
dstack deploy my-service                   # Build + restart + smoke test
dstack audit --pre-commit                  # Quality gate checklist
dstack audit --stale                       # Find stale companion docs
```

## Claude Code Plugin

Install as a Claude Code plugin for skills, hooks, and commands:

```bash
claude plugin install /path/to/dstack/plugin
```

### Skills

| Skill | Purpose |
|-------|---------|
| using-dstack | Session orientation, available commands |
| eruka-memory | When/how to persist learnings |
| multi-repo-ops | Cross-repo dependency awareness |
| vps-deploy | Deployment workflow |
| companion-docs | Track implementation vs plan |
| quality-gates | Pre-commit 5-question checklist |

### Hooks

- **session-start** — Loads project memory automatically
- **quality-gate** — Runs checklist before `git commit`
- **context-monitor** — Warns when context is getting heavy

### Commands

- `/deploy [service]` — Deploy a service
- `/sync [status|dry-run]` — Sync tracked repos
- `/audit [stale]` — Quality audit

## Memory Backends

**File** (default): JSON files at `~/.local/share/dstack/memory/`. Zero dependencies.

**Eruka**: REST API backend for team-shared context memory. Set `$DSTACK_ERUKA_KEY` and configure the URL in config.toml.

## Compatibility

dstack works alongside other Claude Code plugins:

- **superpowers** — TDD, debugging, verification workflows
- **GSD** — Project structure, waves, milestones
- **dstack** — Memory, deployment, multi-repo ops, quality gates

No conflicts. Each handles a different concern.

## Architecture

```
dstack/
├── crates/
│   ├── dstack-memory/     # MemoryProvider trait + backends (file, eruka)
│   └── dstack-cli/        # CLI binary (clap) + library
├── plugin/
│   ├── skills/            # 6 SKILL.md files
│   ├── hooks/             # 3 hook scripts
│   ├── commands/          # 3 command definitions
│   ├── package.json
│   └── CLAUDE.md
├── overlays/              # Private config examples
└── site/                  # Documentation (marmite SSG)
```

## Philosophy

- **Humans verify, AI executes** — Quality gates enforce human checkpoints
- **Memory is a team resource** — Context shouldn't die with a session
- **Friction kills teams before bugs do** — Automate the boring parts
- **Ship the unique value** — Don't rebuild what superpowers/GSD already do

## License

MIT
