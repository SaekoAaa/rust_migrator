# MySQL Migrator Service

A small, single-run MySQL migration utility intended for containers, deployment jobs, and local development. It applies or reverts SQL files, supports credentials from environment variables or mounted secret files, and can optionally export OpenTelemetry traces and metrics.

> **Project status:** early development. The current release executes a fixed set of migration files and does not yet maintain a migration history, verify checksums, or coordinate concurrent instances. Run only one instance at a time and review the [limitations](#current-limitations) before production use.

## Features

- Small Rust binary that exits when the requested operation finishes
- Four migration workflows: apply, revert, apply with seed data, and apply then clear data
- Docker/Kubernetes-style secret file support
- Non-root runtime container
- Optional OTLP/HTTP traces and metrics
- Non-zero exit status on configuration, connection, file, SQL, or telemetry initialization errors

## Quick start

Create a `.env` file from `.env.example`, set the MySQL credentials, then start the example stack:

```sh
docker compose up --build --abort-on-container-exit db_migrator
```

To run the binary directly:

```sh
MYSQL_USER=app \
MYSQL_PASSWORD=secret \
MYSQL_ADDRESS=127.0.0.1 \
MYSQL_PORT=3306 \
MYSQL_DATABASE=app_db \
MIGRATIONS_PATH=./migrations \
MIGRATION_TYPE=1 \
cargo run --release
```

## Migration files

The directory configured by `MIGRATIONS_PATH` may contain these fixed filenames:

| File | Purpose |
| --- | --- |
| `mysql_up.sql` | Create or update the schema |
| `mysql_down.sql` | Revert the schema |
| `mysql_fill_data.sql` | Insert seed data |
| `mysql_drop_data.sql` | Remove seeded data |

`mysql_fill_data.sql` may use a `-- tx;` statement to commit the current data transaction and begin another one.

## Configuration

| Variable | Required | Description |
| --- | --- | --- |
| `MYSQL_ADDRESS` | Yes | MySQL hostname or IP address |
| `MYSQL_PORT` | Yes | MySQL TCP port |
| `MYSQL_DATABASE` | Yes | Database name |
| `MYSQL_USER` | Conditional | Username; takes precedence over `MYSQL_USER_FILE` |
| `MYSQL_PASSWORD` | Conditional | Password; takes precedence over `MYSQL_PASSWORD_FILE` |
| `MYSQL_USER_FILE` | Conditional | File containing the username |
| `MYSQL_PASSWORD_FILE` | Conditional | File containing the password |
| `MIGRATIONS_PATH` | Yes | Directory containing the migration files |
| `MIGRATION_TYPE` | No | Numeric operation, defaults to `1` |
| `WITH_TRACING` | No | Set to `true` to enable OTLP tracing |
| `WITH_METRICS` | No | Set to `true` to enable OTLP metrics |
| `COLLECTOR_URL` | Conditional | OTLP base URL, for example `http://otel:4318/v1` |

At least one source must be available for each credential. The direct variable takes precedence when both it and its corresponding `_FILE` variable are set.

### Operations

| `MIGRATION_TYPE` | Operation | Files executed |
| --- | --- | --- |
| `1` | Apply migration | `mysql_up.sql` |
| `2` | Revert migration | `mysql_down.sql` |
| `3` | Apply with seed data | `mysql_up.sql`, then `mysql_fill_data.sql` |
| `4` | Apply and clear data | `mysql_up.sql`, then `mysql_drop_data.sql` |

## Observability

Tracing and metrics are disabled by default. When enabled, telemetry is sent over OTLP/HTTP to:

- `${COLLECTOR_URL}/traces`
- `${COLLECTOR_URL}/metrics`

The included Compose stack contains an OpenTelemetry Collector, Jaeger, and Prometheus configuration for local evaluation. Database credentials are not included in connection log messages.

## Container deployment

The supplied image runs as UID `10001` without a login shell or home directory. Mount migrations read-only and provide credentials through your platform's secret mechanism. A Kubernetes deployment should run this program as a single Job or controlled deployment hook, not as a replicated long-running service.

Recommended container settings include a read-only root filesystem, disabled privilege escalation, dropped Linux capabilities, explicit CPU/memory requests, and a finite Job retry policy.

## Current limitations

- No migration history table or checksum validation
- No database advisory lock; concurrent executions are unsafe
- SQL is split on semicolons and therefore does not support stored procedures, triggers, custom delimiters, or semicolons inside SQL strings
- MySQL DDL can cause implicit commits, so schema changes are not guaranteed to roll back atomically
- Migration filenames are fixed rather than versioned
- TLS and connection/query timeouts are not yet configurable

These constraints are intentionally documented so operators can decide whether the tool fits their workload.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features
cargo build --release
```

Task aliases are also available through [Task](https://taskfile.dev/):

```sh
task am   # apply
task rm   # revert
task adm  # apply with data
task rac  # apply and clear data
```

## Roadmap

The next reliability milestones are versioned migrations with checksums, migration-state tracking, MySQL advisory locking, a one-connection execution model, integration tests, and configurable TLS/timeouts.
