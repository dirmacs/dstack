# Installing dstack for Pawan

Pawan discovers plugins from `~/.config/pawan/plugins/` or project-level `.pawan/` directories.

## Prerequisites

- [Pawan](https://github.com/dirmacs/pawan) installed
- `dstack` CLI: `cargo install dstack-cli`

## Installation

1. **Clone the dstack repository:**
   ```bash
   git clone https://github.com/dirmacs/dstack.git ~/.config/pawan/plugins/dstack
   ```

2. **Or symlink the plugin directory:**
   ```bash
   mkdir -p ~/.config/pawan/plugins
   ln -s /path/to/dstack/plugin ~/.config/pawan/plugins/dstack
   ```

3. **Restart Pawan** to discover the skills.

## Verify

```bash
ls ~/.config/pawan/plugins/dstack/skills/
```

You should see: companion-docs, eruka-memory, multi-repo-ops, quality-gates, using-dstack, vps-deploy

## Usage

Skills are auto-discovered. Use them in Pawan sessions:

```
/skill using-dstack
/deploy ares
/sync
/audit
```

## Updating

```bash
cd ~/.config/pawan/plugins/dstack && git pull
```

Skills update instantly through the symlink.

## Uninstalling

```bash
rm -rf ~/.config/pawan/plugins/dstack
```
