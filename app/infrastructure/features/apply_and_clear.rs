use std::{fs::read_to_string, path::Path};

use mysql::Pool;
use tracing::debug_span;

use crate::infrastructure::apply_tx::apply_transaction;

///Apply "CREATE" tables in transaction and drop queries in transaction
#[tracing::instrument(skip(pool))]
pub fn apply_and_clear_data(
    pool: &Pool,
    path_to_create_file: &Path,
    path_to_clear_tables_file: &Path,
) -> anyhow::Result<()> {
    let span = debug_span!("applying_migration", path = &path_to_create_file.to_str());
    span.in_scope(|| {
        let sql = read_to_string(path_to_create_file).inspect_err(|e| {
            tracing::error!(
                error = %e,
                "Failed to read migration file at path: {}",
                path_to_create_file.display()
            )
        })?;
        apply_transaction(pool, &sql)?;
        Ok::<_, anyhow::Error>(())
    })?;

    let span = debug_span!(
        "Clearing mysql data",
        path = &path_to_clear_tables_file.to_str()
    );
    span.in_scope(|| {
        let clear_data_sql = read_to_string(path_to_clear_tables_file).inspect_err(|e| {
            tracing::error!(
                error = %e,
                "Failed to read drop data file at path: {}",
                path_to_clear_tables_file.display()
            )
        })?;
        apply_transaction(pool, &clear_data_sql)?;
        Ok::<_, anyhow::Error>(())
    })?;
    Ok(())
}
