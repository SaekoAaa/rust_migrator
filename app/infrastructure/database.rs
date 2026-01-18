use mysql::{Opts, Pool};
use tracing::info;
#[tracing::instrument]
pub fn connect_to_database(mysql_connection_string: &str) -> anyhow::Result<Pool> {
    let opts = Opts::try_from(mysql_connection_string)?;
    let pool = Pool::new(opts)?;
    info!("Connected to database!");
    Ok(pool)
}
