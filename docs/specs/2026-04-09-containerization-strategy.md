# VPS Containerization Strategy

## Current State

Everything runs as systemd services with static Rust binaries. No containers until ix installation (Apr 9).

| Service | Port | Binary | State |
|---------|------|--------|-------|
| ARES | 3000 | ares-dirmacs | systemd, static binary |
| Eruka | 8081 | eruka-api | systemd, static binary |
| eHB | 8090 | ehb-server | systemd, static binary |
| DolTARES | 3100 | doltares | systemd, static binary |
| Caddy | 80/443 | caddy | systemd, package |
| PostgreSQL | 5432 | postgres | systemd, package |
| channel-gateway | 9000 | channel-gateway | systemd, Go binary |
| daedra | 3400 | daedra | systemd, static binary |
| dirmacs-site | 3200 | next-server | systemd, Node |
| ix memory-layer | (wants 8090) | Docker container | CONFLICT |

## Decision: Hybrid Strategy

**Keep as static binaries (systemd):**
- ARES, Eruka, eHB, DolTARES, daedra, channel-gateway — these are our Rust/Go binaries. They compile to single executables, start in <1s, use minimal memory, and benefit from direct filesystem access. Containerizing adds overhead with no benefit.
- Caddy — system package, manages TLS certs on disk. Container adds complexity.
- PostgreSQL — system package with data on disk. Container adds risk to data persistence.

**Run as containers (docker-compose):**
- ix memory-layer + ArangoDB — third-party software we don't build. Containers are the right isolation boundary.
- Any future third-party services (Redis, S3-compatible storage, etc.)
- dirmacs-site (Next.js) — could move to container for cleaner Node isolation, but low priority.

## Port Assignment (permanent)

| Port | Service | Notes |
|------|---------|-------|
| 80/443 | Caddy | Public, TLS |
| 3000 | ARES | Internal, Caddy proxies |
| 3100 | DolTARES | Internal |
| 3200 | dirmacs-site | Internal, Caddy proxies |
| 3400 | daedra | Internal |
| 3500 | dstack-server | Internal (new) |
| 5432 | PostgreSQL | Internal |
| 8081 | Eruka | Internal, Caddy proxies |
| 8090 | eHB | Internal, Caddy proxies |
| 8529 | ArangoDB (ix) | Container, internal |
| 8095 | ix memory-layer | Container, remapped from 8090 |
| 9000 | channel-gateway | Internal |

## ix Resolution

ix memory-layer runs inside Docker on port 8090 internally, mapped to host port **8095** to avoid conflict with eHB. The ix CLI health check hardcodes 8090 — we set `IX_MEMORY_URL=http://localhost:8095` in `~/.bashrc` and `~/.config/dstack/.env`.

## docker-compose.yml (for third-party services)

Location: `~/.ix/backend/docker-compose.yml`

Modified to map port 8095:8090 instead of 8090:8090.

## What This Means for aegis

aegis manages configs for the static binary services. Docker services are managed by docker-compose. These are separate concern domains:
- **aegis** → systemd services, env files, binary symlinking
- **docker-compose** → container services (ix, future third-party)
- **dstack** → orchestrates both (deploy for systemd, future docker support)

## Future: dstack deploy for containers

v0.4 could add container awareness to `dstack deploy`:
```toml
[deploy.ix]
type = "docker-compose"
compose_file = "~/.ix/backend/docker-compose.yml"
smoke = "curl -sf http://localhost:8095/v1/health"
```
