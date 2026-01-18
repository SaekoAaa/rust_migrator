use std::{fs::read_to_string, path::Path};

use mysql::Pool;

use crate::infrastructure::apply_tx::apply_transaction;

///Apply "CREATE" tables in transaction
#[tracing::instrument(skip(pool))]
pub fn apply_migration(pool: &Pool, migration_path: &Path) -> anyhow::Result<()> {
    tracing::info!("Applying migrations");

    let sql = read_to_string(&migration_path).inspect_err(|e| {
        tracing::error!(
            error = %e,
            "Failed to read migration file at path: {}",
            migration_path.display(),
        )
    })?;
    apply_transaction(&pool, &sql)?;
    Ok(())
}
