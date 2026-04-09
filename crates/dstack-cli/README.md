# dstack-cli

Development stack CLI for AI-assisted multi-repo work.

## Install

```bash
cargo install dstack-cli
```

## Commands

| Command | Description |
|---------|-------------|
| `dstack memory load` | Load memory fields for a project |
| `dstack memory save <key> <value>` | Save a key-value pair to memory |
| `dstack memory query <pattern>` | Search memory by keyword |
| `dstack memory export` | Export all memory as JSON |
| `dstack deploy <service>` | Deploy a service (build + restart + smoke test) |
| `dstack deploy --all` | Deploy all configured services |
| `dstack sync` | Git sync across tracked repos |
| `dstack sync --status` | Show sync status without pushing |
| `dstack skills list` | List available skills |
| `dstack skills sync` | Install all skills |
| `dstack audit` | Quality audit summary |
| `dstack audit --pre-commit` | Pre-commit quality gate |
| `dstack init <dir>` | Scaffold a multi-platform plugin |

## Configuration

Create `~/.config/dstack/config.toml`:

```toml
[memory]
backend = "file"  # or "eruka"

[repos]
tracked = ["~/projects/my-repo"]

[deploy.my-service]
build = "cargo build --release"
service = "my-service"
smoke = "curl -sf http://localhost:3000/health"

[deploy.my-containers]
deploy_type = "docker-compose"
service = "my-service"
compose_file = "~/docker-compose.yml"
smoke = "curl -sf http://localhost:8080/health"
```

## Plugin

dstack includes a multi-platform plugin for AI coding agents. See the [plugin README](../../plugin/README.md).

## License

MIT
