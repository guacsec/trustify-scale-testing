extern crate core;

mod graphql;

use crate::graphql::*;
use goose::prelude::*;
use std::time::Duration;

/// Define a transaction and use its function identifier as name
macro_rules! tx {
    ($n:ident) => {
        transaction!($n).set_name(stringify!($n))
    };
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let wait_time_from: u64 = std::env::var("WAIT_TIME_FROM")
        .map(|s| s.parse().unwrap_or(5))
        .unwrap_or(5);
    let wait_time_to: u64 = std::env::var("WAIT_TIME_TO")
        .map(|s| s.parse().unwrap_or(15))
        .unwrap_or(15);

    GooseAttack::initialize()?
        .register_scenario(
            scenario!("GraphQLUser")
                .set_wait_time(
                    Duration::from_secs(wait_time_from),
                    Duration::from_secs(wait_time_to),
                )?
                .register_transaction(tx!(graphql_query_advisory)),
        )
        .execute()
        .await?;

    Ok(())
}
