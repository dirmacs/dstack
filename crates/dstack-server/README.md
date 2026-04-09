# dstack-server

HTTP API server mirroring all dstack CLI commands. Built with Axum.

## Usage

```bash
# Start the server (default: 127.0.0.1:3500)
dstack-serve

# Custom port and bind address
dstack-serve --port 8080 --bind 0.0.0.0
```

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/config` | Current configuration |
| POST | `/memory/load` | Load memory fields |
| POST | `/memory/save` | Save a key-value pair |
| POST | `/memory/query` | Search memory |
| GET | `/memory/export` | Export all memory |
| POST | `/sync/status` | Repo sync status |
| POST | `/audit` | Quality audit |
| POST | `/audit/stale` | Stale companion docs |
| GET | `/skills` | List available skills |
| POST | `/skills/install` | Install a skill |
| POST | `/skills/sync` | Sync all skills |

## License

MIT
