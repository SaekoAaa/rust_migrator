# MySQL Migrator

A focused MySQL migration runner for containers, Kubernetes Jobs, deployment hooks, and small services. It ships as a single Rust binary, uses one database connection, tracks applied schema versions, supports mounted secrets, and exits as soon as its work is complete.

| Artifact | Size |
| --- | ---: |
| Optimized release binary | **6.01 MB** |
| Minimal container image | **6.11 MB** |

*Lightweight and popular sqlx-cli image size is 23 MB as example

The default build includes Rustls-based MySQL TLS support without bundling the optional OpenTelemetry exporter stack.

## Why this project

- **Predictable footprint:** one MySQL connection and no long-running service
- **Deployment-friendly:** standalone binary and non-root Alpine image
- **Traceable changes:** successful operations are recorded in `schema_versions`
- **Secret-aware:** credentials can come from environment variables or mounted files
- **Observable when needed:** OTLP traces and metrics are a compile-time feature
- **Automation-ready:** non-zero failure exits, GHCR publishing, and immutable version tags
- **Safe logging:** database passwords are never included in connection logs

## Quick start

### Run the binary

Download the release binary and make it executable:

```sh
chmod +x migrator_service

MYSQL_USER=app \
MYSQL_PASSWORD=secret \
MYSQL_ADDRESS=127.0.0.1 \
MYSQL_PORT=3306 \
MYSQL_DATABASE=app_db \
MIGRATIONS_PATH=./migrations \
MIGRATION_TYPE=1 \
./migrator_service
```

The process exits with code `0` after success and a non-zero code after a configuration, connection, file, SQL, or telemetry initialization failure.

### Run the container

```sh
docker run --rm \
  -e MYSQL_USER=app \
  -e MYSQL_PASSWORD=secret \
  -e MYSQL_ADDRESS=mysql.example.internal \
  -e MYSQL_PORT=3306 \
  -e MYSQL_DATABASE=app_db \
  -e MIGRATIONS_PATH=/migrations \
  -e MIGRATION_TYPE=1 \
  -v "$PWD/migrations:/migrations:ro" \
  ghcr.io/saekoaaa/migrator-service:latest
```

The image runs as UID `10001` without a login shell or home directory.

## Migration modes

The directory configured by `MIGRATIONS_PATH` uses four explicit files:

| `MIGRATION_TYPE` | Operation | Files executed |
| ---: | --- | --- |
| `1` | Apply schema | `mysql_up.sql` |
| `2` | Revert schema | `mysql_down.sql` |
| `3` | Apply schema and seed data | `mysql_up.sql`, `mysql_fill_data.sql` |
| `4` | Apply schema and clear data | `mysql_up.sql`, `mysql_drop_data.sql` |

`MIGRATION_TYPE` defaults to `1` when omitted.

### Split large data migrations

Data migrations can be divided into smaller transactions with a `-- tx;` boundary:

```sql
INSERT INTO users (email, password_hash, role)
VALUES ('admin@example.com', 'example-hash', 'admin');

-- tx;

INSERT INTO projects (owner_id, name, valid_name, description)
VALUES (1, 'Example', 'example', 'Example project');
```

The current transaction is committed at each boundary and a new transaction is opened for the following statements. This is useful for controlling transaction size in seed-data workloads.

## Schema version tracking

The migrator automatically creates a `schema_versions` table. A successful apply mode records the next numeric version, migration mode, executed files, and timestamp. Revert requires an existing version and removes the latest version record only after `mysql_down.sql` succeeds.

```sql
CREATE TABLE schema_versions (
    version BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    migration_mode VARCHAR(32) NOT NULL,
    migration_files VARCHAR(512) NOT NULL,
    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## Configuration

| Variable | Required | Description |
| --- | --- | --- |
| `MYSQL_ADDRESS` | Yes | MySQL hostname or IP address |
| `MYSQL_PORT` | Yes | MySQL TCP port |
| `MYSQL_DATABASE` | Yes | Database name |
| `MYSQL_USER` | Conditional | Username; preferred over `MYSQL_USER_FILE` |
| `MYSQL_PASSWORD` | Conditional | Password; preferred over `MYSQL_PASSWORD_FILE` |
| `MYSQL_USER_FILE` | Conditional | File containing the username |
| `MYSQL_PASSWORD_FILE` | Conditional | File containing the password |
| `MIGRATIONS_PATH` | Yes | Directory containing migration files |
| `MIGRATION_TYPE` | No | Migration mode; defaults to `1` |

At least one source must be provided for each credential. Direct variables take precedence over their corresponding `_FILE` variables.

## Optional OpenTelemetry

Telemetry dependencies are excluded from the minimal build. Compile them explicitly when OTLP traces or metrics are required:

```sh
cargo build --release --locked --features telemetry
```

For a telemetry-enabled container:

```sh
docker build \
  --build-arg "CARGO_FEATURES=--features telemetry" \
  -t migrator-service:telemetry \
  .
```

Runtime configuration:

| Variable | Description |
| --- | --- |
| `WITH_TRACING=true` | Export OTLP/HTTP traces |
| `WITH_METRICS=true` | Export OTLP/HTTP metrics |
| `COLLECTOR_URL` | OTLP base URL, such as `http://otel:4318/v1` |

The included Compose stack provides OpenTelemetry Collector, Jaeger, and Prometheus configurations for local evaluation.

## Build and publish

Build the optimized minimal binary:

```sh
cargo build --release --locked
```

Build the container:

```sh
docker build --build-arg RUST_VERSION=1.89 -t migrator-service .
```

Publish a version to GHCR through the Taskfile:

```sh
task push_package -- v1.2.3
```

Pushing a `v*` Git tag also triggers the GitHub Actions workflow and publishes version, commit-SHA, and `latest` image tags.

## Development checks

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
```

## Operational boundaries

- Run a single migrator instance at a time; advisory locking is not implemented yet.
- Migration files have fixed names and do not currently store checksums.
- SQL is split on semicolons, so stored procedures, custom delimiters, and semicolons inside strings are not supported.
- MySQL may implicitly commit DDL statements; schema rollback is therefore not guaranteed to be atomic.

These boundaries keep the runner intentionally compact and are documented so operators can evaluate it against their deployment requirements.
