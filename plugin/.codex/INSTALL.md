# Installing dstack for Codex

Enable dstack skills in Codex via native skill discovery. Clone and symlink.

## Prerequisites

- Git
- `dstack` CLI: `cargo install dstack-cli`

## Installation

1. **Clone the dstack repository:**
   ```bash
   git clone https://github.com/dirmacs/dstack.git ~/.codex/dstack
   ```

2. **Create the skills symlink:**
   ```bash
   mkdir -p ~/.agents/skills
   ln -s ~/.codex/dstack/plugin/skills ~/.agents/skills/dstack
   ```

   **Windows (PowerShell):**
   ```powershell
   New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.agents\skills"
   cmd /c mklink /J "$env:USERPROFILE\.agents\skills\dstack" "$env:USERPROFILE\.codex\dstack\plugin\skills"
   ```

3. **Restart Codex** (quit and relaunch the CLI) to discover the skills.

## Verify

```bash
ls -la ~/.agents/skills/dstack
```

You should see a symlink pointing to the dstack skills directory.

## Updating

```bash
cd ~/.codex/dstack && git pull
```

Skills update instantly through the symlink.

## Uninstalling

```bash
rm ~/.agents/skills/dstack
```

Optionally delete the clone: `rm -rf ~/.codex/dstack`.
