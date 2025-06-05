use sqlx::{ConnectOptions, postgres::PgConnectOptions};
use std::str::FromStr;

/// create a new database connection
pub(crate) async fn connect(db: &str) -> anyhow::Result<sqlx::PgConnection> {
    Ok(PgConnectOptions::from_str(db)?.connect().await?)
}
