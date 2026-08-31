use std::{fs::read_to_string, path::Path};

use mysql::Pool;

use crate::infrastructure::apply_tx::apply_transaction;

#[tracing::instrument(skip(pool))]
pub fn revert_migration(pool: &Pool, migrations_path: &Path) -> anyhow::Result<()> {
    tracing::info!("Reverting migrations");
    let sql = read_to_string(migrations_path).inspect_err(|_| {
        tracing::error!(
            "Failed to read migration file at path: {}",
            migrations_path.display()
        )
    })?;
    apply_transaction(pool, &sql)?;
    Ok(())
}
