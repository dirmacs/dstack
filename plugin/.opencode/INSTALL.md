# Installing dstack for OpenCode

## Prerequisites

- [OpenCode.ai](https://opencode.ai) installed
- `dstack` CLI: `cargo install dstack-cli`

## Installation

Add dstack to the `plugin` array in your `opencode.json` (global or project-level):

```json
{
  "plugin": ["dstack@git+https://github.com/dirmacs/dstack.git#plugin"]
}
```

Restart OpenCode. The plugin auto-installs and registers all skills.

Verify by asking: "What dstack skills are available?"

## Usage

Use OpenCode's native `skill` tool:

```
use skill tool to list skills
use skill tool to load dstack/using-dstack
use skill tool to load dstack/quality-gates
```

## Tool Mapping

When skills reference Claude Code tools:
- `Bash` → `bash`
- `Read` → `read`
- `Edit` → `edit`
- `Skill` tool → OpenCode's native `skill` tool
- File operations → your native tools

## Updating

dstack updates automatically when you restart OpenCode.

To pin a specific version:

```json
{
  "plugin": ["dstack@git+https://github.com/dirmacs/dstack.git#v0.2.0"]
}
```

## Getting Help

- Report issues: https://github.com/dirmacs/dstack/issues
- Documentation: https://dirmacs.github.io/dstack
