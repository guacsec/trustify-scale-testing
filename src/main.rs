// The simplest loadtest example
mod db;
mod graphql;
mod oidc;
mod restapi;
mod scenario;
mod website;

use crate::{
    // graphql::*,
    oidc::{OpenIdTokenProvider, OpenIdTokenProviderConfigArguments},
    restapi::*,
    website::*,
};
use anyhow::Context;
use goose::prelude::*;
use std::{str::FromStr, sync::Arc, time::Duration};

/// Define a transaction and use its function identifier as name
macro_rules! tx {
    ($n:ident) => {
        transaction!($n).set_name(stringify!($n))
    };
    ($n:ident($v1:expr)) => {{
        let v1 = ($v1).clone();
        Transaction::new(Arc::new({
            let v1 = v1.clone();
            move |s| Box::pin($n(v1.clone(), s))
        }))
        .set_name(&format!("{}[{v1}]", stringify!($n)))
    }};
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let wait_time_from: u64 = std::env::var("WAIT_TIME_FROM")
        .map(|s| s.parse().unwrap_or(5))
        .unwrap_or(5);
    let wait_time_to: u64 = std::env::var("WAIT_TIME_TO")
        .map(|s| s.parse().unwrap_or(15))
        .unwrap_or(15);

    let scenario_file = std::env::var("SCENARIO_FILE").ok();

    if !matches!(
        std::env::var("GENERATE_SCENARIO").ok().as_deref(),
        Some("false" | "0")
    ) {
        let scenario = scenario::Scenario::eval().await?;
        println!("{}", serde_json5::to_string(&scenario)?);
        return Ok(());
    }

    let scenario = Arc::new(scenario::Scenario::load(scenario_file.as_deref()).await?);

    let provider = create_oidc_provider().await?;
    let custom_client = Transaction::new(Arc::new(move |user| {
        let provider = provider.clone();
        Box::pin(async move { setup_custom_client(&provider, user).await })
    }));

    GooseAttack::initialize()?
        .test_start(
            Transaction::new(Arc::new({
                let scenario = scenario.clone();
                move |_| {
                    let scenario = scenario.clone();
                    Box::pin(async move {
                        log::info!("Scenario: {scenario:#?}");
                        Ok(())
                    })
                }
            }))
            .set_name("log scenario"),
        )
        .register_scenario(
            scenario!("WebsiteUser")
                // .set_weight(1)?
                .register_transaction(custom_client.clone().set_name("logon"))
                // After each transactions runs, sleep randomly from 5 to 15 seconds.
                .set_wait_time(
                    Duration::from_secs(wait_time_from),
                    Duration::from_secs(wait_time_to),
                )?
                .register_transaction(tx!(website_index))
                .register_transaction(tx!(website_openapi))
                .register_transaction(tx!(website_sboms))
                .register_transaction(tx!(website_packages))
                .register_transaction(tx!(website_advisories))
                .register_transaction(tx!(website_importers)),
        )
        .register_scenario({
            let mut s = scenario!("RestAPIUser")
                // .set_weight(1)?
                .register_transaction(custom_client.clone().set_name("logon"))
                // After each transactions runs, sleep randomly from 5 to 15 seconds.
                .set_wait_time(
                    Duration::from_secs(wait_time_from),
                    Duration::from_secs(wait_time_to),
                )?
                .register_transaction(tx!(list_organizations))
                .register_transaction(tx!(list_advisory))
                .register_transaction(tx!(list_advisory_paginated))
                .register_transaction(tx!(get_advisory_by_doc_id))
                .register_transaction(tx!(list_vulnerabilities))
                .register_transaction(tx!(list_vulnerabilities_paginated))
                .register_transaction(tx!(list_importer))
                .register_transaction(tx!(list_packages))
                .register_transaction(tx!(list_packages_paginated))
                .register_transaction(tx!(search_packages))
                .register_transaction(tx!(search_exact_packages))
                .register_transaction(tx!(list_products))
                .register_transaction(tx!(list_sboms))
                .register_transaction(tx!(list_sboms_paginated));

            if let Some(large_sbom) = scenario.large_sbom.clone() {
                s = s
                    .register_transaction(tx!(get_sbom_info(large_sbom.clone())))
                    .register_transaction(tx!(get_sbom_advisories(large_sbom.clone())))
                    .register_transaction(tx!(get_sbom_packages(large_sbom.clone())))
                    .register_transaction(tx!(get_sbom_related(large_sbom.clone())))
            }

            s
        })
        // .register_scenario(
        //     scenario!("GraphQLUser")
        //         // .set_weight(1)?
        //         .register_transaction(custom_client.set_name("logon"))
        //         // After each transactions runs, sleep randomly from 5 to 15 seconds.
        //         .set_wait_time(
        //             Duration::from_secs(wait_time_from),
        //             Duration::from_secs(wait_time_to),
        //         )?
        //         .register_transaction(tx!(g_get_advisories))
        //         .register_transaction(tx!(g_get_advisory_by_id))
        //         .register_transaction(tx!(g_get_organization_by_name))
        //         .register_transaction(tx!(g_get_sbom_by_id))
        //         .register_transaction(tx!(g_get_sbom_by_labels))
        //         .register_transaction(tx!(g_cves_by_sbom))
        //         .register_transaction(tx!(g_get_vulnerability_by_id))
        //         .register_transaction(tx!(g_get_vulnerabilities)),
        // )
        .execute()
        .await?;

    Ok(())
}

async fn create_oidc_provider() -> anyhow::Result<OpenIdTokenProvider> {
    let issuer_url = std::env::var("ISSUER_URL").context("Missing env-var 'ISSUER_URL'")?;
    let client_id = std::env::var("CLIENT_ID").context("Missing env-var 'CLIENT_ID'")?;
    let client_secret =
        std::env::var("CLIENT_SECRET").context("Missing env-var 'CLIENT_SECRET'")?;
    let refresh_before = std::env::var("OIDC_REFRESH_BEFORE").unwrap_or_else(|_| "30s".to_string());
    let refresh_before =
        humantime::Duration::from_str(&refresh_before).context("OIDC_REFRESH_BEFORE must parse")?;

    let provider = OpenIdTokenProvider::with_config(OpenIdTokenProviderConfigArguments {
        client_id,
        client_secret,
        issuer_url,
        refresh_before,
        tls_insecure: false,
    })
    .await
    .context("discover OIDC client")?;

    Ok(provider)
}

// required until https://github.com/tag1consulting/goose/pull/605 is merged
#[allow(clippy::expect_used)]
async fn setup_custom_client(
    provider: &OpenIdTokenProvider,
    user: &mut GooseUser,
) -> TransactionResult {
    set_custom_client(provider, user)
        .await
        .expect("Failed to set up client");
    Ok(())
}

async fn set_custom_client(
    provider: &OpenIdTokenProvider,
    user: &mut GooseUser,
) -> anyhow::Result<()> {
    use reqwest::{Client, header};

    log::debug!("Creating a new custom client");

    let auth_token: String = provider
        .provide_token()
        .await
        .context("get OIDC token")?
        .access_token;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&format!("Bearer {auth_token}"))?,
    );

    let timeout = std::env::var("REQUEST_TIMEOUT")
        .ok()
        .map(|value| humantime::Duration::from_str(&value))
        .transpose()?
        .map(|v| v.into())
        .unwrap_or(Duration::from_secs(300));

    // Build a custom client.
    let builder = Client::builder()
        .default_headers(headers)
        .user_agent("loadtest-ua")
        .timeout(timeout);

    // Assign the custom client to this GooseUser.
    user.set_client_builder(builder).await?;

    Ok(())
}
