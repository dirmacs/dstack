# dstack

Development stack for AI-assisted multi-repo work. Persistent memory, quality gates, VPS deployment, cross-repo sync.

Built by [DIRMACS](https://dirmacs.com). Born from production pain across 15+ repos.

## Installation

Installation differs by platform. Claude Code and Cursor have built-in plugin systems. Codex and OpenCode require manual setup.

### Claude Code (Official Marketplace)

```
/plugin install dstack@dirmacs-marketplace
```

Or register the marketplace first:

```
/plugin marketplace add dirmacs/dstack-marketplace
/plugin install dstack@dstack-marketplace
```

### Cursor (Plugin Marketplace)

In Cursor Agent chat:

```
/add-plugin dstack
```

Or search for "dstack" in the plugin marketplace.

### Codex

Tell Codex:

```
Fetch and follow instructions from https://raw.githubusercontent.com/dirmacs/dstack/refs/heads/main/plugin/.codex/INSTALL.md
```

Or manually:

```bash
git clone https://github.com/dirmacs/dstack.git ~/.codex/dstack
mkdir -p ~/.agents/skills
ln -s ~/.codex/dstack/plugin/skills ~/.agents/skills/dstack
```

### OpenCode

Tell OpenCode:

```
Fetch and follow instructions from https://raw.githubusercontent.com/dirmacs/dstack/refs/heads/main/plugin/.opencode/INSTALL.md
```

Or add to `opencode.json`:

```json
{
  "plugin": ["dstack@git+https://github.com/dirmacs/dstack.git#plugin"]
}
```

### GitHub Copilot CLI

```
copilot plugin marketplace add dirmacs/dstack-marketplace
copilot plugin install dstack@dstack-marketplace
```

### Gemini CLI

```
gemini extensions install https://github.com/dirmacs/dstack
```

To update:

```
gemini extensions update dstack
```

## Prerequisites

The plugin works standalone for skills and quality gates. For full functionality, install the `dstack` CLI:

```bash
cargo install dstack-cli
```

Then create `~/.config/dstack/config.toml`:

```toml
[memory]
backend = "file"  # or "eruka" for persistent cross-session memory

[repos]
tracked = ["~/projects/my-repo", "~/projects/another-repo"]

[deploy.my-service]
build = "cd ~/projects/my-repo && cargo build --release"
service = "my-service"
smoke = "curl -sf http://localhost:3000/health"
```

## What's Included

### Skills

| Skill | Purpose |
|-------|---------|
| using-dstack | Session orientation, available commands |
| eruka-memory | When and how to persist learnings across sessions |
| multi-repo-ops | Cross-repo dependency awareness and sync |
| vps-deploy | Build + restart + smoke test deployment |
| companion-docs | Track implementation reality vs plan intent |
| quality-gates | Pre-commit 5-question checklist |

### Commands

| Command | Purpose |
|---------|---------|
| `/deploy [service]` | Deploy a service or all tracked services |
| `/sync` | Sync all tracked repos |
| `/audit` | Run quality gate checklist |

### Hooks

| Hook | Trigger | Purpose |
|------|---------|---------|
| session-start | SessionStart | Load dstack config + memory context |
| quality-gate | PreToolUse (Bash) | Pre-commit 5-question checklist |
| proprietary-guard | PreToolUse (Bash) | Block proprietary content in public repos |
| context-monitor | PostToolUse | Track token usage and context health |

## Rust CLI

dstack is also a Rust CLI and library:

```bash
# CLI
cargo install dstack-cli

# Library (for building integrations)
cargo add dstack-memory
```

Crates: [dstack-cli](https://crates.io/crates/dstack-cli) | [dstack-memory](https://crates.io/crates/dstack-memory)

Docs: [dirmacs.github.io/dstack](https://dirmacs.github.io/dstack)

## License

MIT
