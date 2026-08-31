use mysql::{Pool, params, prelude::Queryable};

const CREATE_SCHEMA_VERSIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS schema_versions (
    version BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    migration_mode VARCHAR(32) NOT NULL,
    migration_files VARCHAR(512) NOT NULL,
    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
"#;

#[tracing::instrument(skip(pool))]
pub fn ensure_table(pool: &Pool) -> anyhow::Result<()> {
    let mut connection = pool.get_conn()?;
    connection.query_drop(CREATE_SCHEMA_VERSIONS_TABLE)?;
    Ok(())
}

#[tracing::instrument(skip(pool))]
pub fn latest_version(pool: &Pool) -> anyhow::Result<Option<u64>> {
    let mut connection = pool.get_conn()?;
    let version: Option<Option<u64>> =
        connection.query_first("SELECT MAX(version) FROM schema_versions")?;
    Ok(version.flatten())
}

#[tracing::instrument(skip(pool))]
pub fn record_applied(
    pool: &Pool,
    migration_mode: &str,
    migration_files: &str,
) -> anyhow::Result<u64> {
    let version = latest_version(pool)?.unwrap_or(0) + 1;
    let mut connection = pool.get_conn()?;
    connection.exec_drop(
        r#"
        INSERT INTO schema_versions (version, migration_mode, migration_files)
        VALUES (:version, :migration_mode, :migration_files)
        "#,
        params! {
            "version" => version,
            "migration_mode" => migration_mode,
            "migration_files" => migration_files,
        },
    )?;
    tracing::info!(schema.version = version, "Recorded schema version");
    Ok(version)
}

#[tracing::instrument(skip(pool))]
pub fn remove_version(pool: &Pool, version: u64) -> anyhow::Result<()> {
    let mut connection = pool.get_conn()?;
    connection.exec_drop(
        "DELETE FROM schema_versions WHERE version = :version",
        params! { "version" => version },
    )?;

    if connection.affected_rows() != 1 {
        anyhow::bail!("Schema version {version} was not found");
    }

    tracing::info!(schema.version = version, "Removed reverted schema version");
    Ok(())
}
