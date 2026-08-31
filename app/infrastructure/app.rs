use std::path::Path;

use opentelemetry::KeyValue;

use crate::{
    infrastructure::{
        database::connect_to_database,
        features::{
            apply_and_clear::apply_and_clear_data, apply_migration::apply_migration,
            apply_with_insert::apply_with_insert, revert_migration::revert_migration,
        },
        load_env::{Enviroment, MigrationType},
    },
    observability::metrics::{MIGRATIONS_COUNTER, MIGRATIONS_DURATION},
};

#[tracing::instrument(level = "info")]
pub fn run() -> anyhow::Result<()> {
    let env = Enviroment::load_env().inspect_err(|e| {
        tracing::error!(error = %e, "Failed to load environment");
    })?;
    let database_port = env
        .database_port
        .parse::<u16>()
        .map_err(|error| anyhow::anyhow!("Invalid MYSQL_PORT '{}': {error}", env.database_port))?;
    tracing::info!(
        database.address = %env.database_address,
        database.port = database_port,
        database.name = %env.database,
        database.user = %env.mysql_user,
        "Connecting to database"
    );
    let pool = connect_to_database(
        &env.database_address,
        database_port,
        &env.database,
        &env.mysql_user,
        &env.mysql_password,
    )
        .inspect_err(|error| tracing::error!(%error, database = env.database, database.user = env.mysql_user, database.address = env.database_address, database.port = env.database_port, "Connecting to database error"))?;
    let migration_path = Path::new(&env.migrations_path);
    let timer = std::time::Instant::now();
    match env.migration_type {
        MigrationType::RevertMigration => {
            revert_migration(&pool, &migration_path.join("mysql_down.sql"))?
        }
        MigrationType::ApplyMigration => {
            apply_migration(&pool, &migration_path.join("mysql_up.sql"))?
        }
        MigrationType::ApplyWithData => {
            apply_with_insert(
                &pool,
                &migration_path.join("mysql_up.sql"),
                &migration_path.join("mysql_fill_data.sql"),
            )?;
        }
        MigrationType::ApplyAndClearData => {
            apply_and_clear_data(
                &pool,
                &migration_path.join("mysql_up.sql"),
                &migration_path.join("mysql_drop_data.sql"),
            )?;
        }
    };

    let kv = &[KeyValue::new(
        "migration_mode",
        Into::<&'static str>::into(env.migration_type),
    )];
    if let Some(migrations_counter) = MIGRATIONS_COUNTER.get() {
        migrations_counter.add(1, kv);
    }
    if let Some(migrations_duration) = MIGRATIONS_DURATION.get() {
        migrations_duration.record(timer.elapsed().as_secs_f64(), kv);
    }
    tracing::info!("Process finished");
    Ok(())
}
