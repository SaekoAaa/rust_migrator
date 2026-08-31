use std::{fs::read_to_string, path::Path};

use mysql::Pool;
use tracing::debug_span;

use crate::infrastructure::apply_tx::{apply_separately, apply_transaction};

/// Applies "CREATE" queries in transaction and separately applies "INSERT" chunks split by "-- tx"
#[tracing::instrument(skip(pool))]
pub fn apply_with_insert(
    pool: &Pool,
    path_to_create_table_file: &Path,
    path_to_insert_sql_file: &Path,
) -> anyhow::Result<()> {
    let span = debug_span!(
        "applying_migration",
        path = &path_to_create_table_file.to_str()
    );
    span.in_scope(|| {
        let sql = read_to_string(path_to_create_table_file).inspect_err(|e| {
            tracing::error!(
                error = %e,
                "Failed to read migration file at path: {}",
                path_to_create_table_file.display()
            )
        })?;
        apply_transaction(pool, &sql)?;
        Ok::<_, anyhow::Error>(())
    })?;

    let span = debug_span!("Filling sql data", path = &path_to_insert_sql_file.to_str());
    span.in_scope(|| {
        let fill_data_sql = read_to_string(path_to_insert_sql_file).inspect_err(|e| {
            tracing::error!(
                error = %e,
                "Failed to read fill data file at path: {}",
                path_to_insert_sql_file.display()
            )
        })?;
        apply_separately(pool, &fill_data_sql)?;
        Ok::<_, anyhow::Error>(())
    })?;
    Ok(())
}
