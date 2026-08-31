use mysql::{OptsBuilder, Pool};
use tracing::info;
#[tracing::instrument(skip(password))]
pub fn connect_to_database(
    address: &str,
    port: u16,
    database: &str,
    user: &str,
    password: &str,
) -> anyhow::Result<Pool> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(address))
        .tcp_port(port)
        .db_name(Some(database))
        .user(Some(user))
        .pass(Some(password));
    let pool = Pool::new(opts)?;
    info!("Connected to database!");
    Ok(pool)
}
