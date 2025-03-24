use sqlx::{
    ConnectOptions,
    any::{AnyConnectOptions, install_default_drivers},
};
use std::str::FromStr;

/// create a new database connection
pub(crate) async fn connect(db: &str) -> anyhow::Result<sqlx::AnyConnection> {
    install_default_drivers();
    Ok(AnyConnectOptions::from_str(db)?.connect().await?)
}
