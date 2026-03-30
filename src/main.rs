// The simplest loadtest example
mod db;
mod oidc;
mod restapi;
mod scenario;
mod utils;
mod website;

use crate::{
    oidc::{OpenIdTokenProvider, OpenIdTokenProviderConfigArguments},
    restapi::{
        advisory::*, analysis::*, misc::*, purl::*, sbom::*, sbom_group::*, vulnerability::*,
    },
    website::*,
};
use anyhow::Context;
use goose::prelude::*;
use std::{str::FromStr, sync::Arc, time::Duration};

const MAX_ID_DISPLAY: usize = 32;

/// Creates a named goose [`Transaction`] that issues a single `GET` request.
///
/// If `query` is non-empty the URL becomes `path?query`, otherwise just `path`.
/// The transaction's metric name is set to the resulting URL, giving every
/// variant its own stable row in the load-test report.
///
/// # Arguments
/// * `path`  — The URL path, e.g. `"/api/v2/advisory"`.
/// * `query` — The pre-encoded query string without the leading `?`,
///   e.g. `"q=title~openssl"`, or `""` for no query string.
fn list_tx(path: &'static str, query: &'static str) -> Transaction {
    let url = if query.is_empty() {
        path.to_string()
    } else {
        format!("{}?{}", path, query)
    };
    Transaction::new(Arc::new({
        let url = url.clone();
        move |user| {
            let url = url.clone();
            Box::pin(async move {
                let _response = user.get(&url).await?;
                Ok(())
            })
        }
    }))
    .set_name(&url)
}

/// Define a transaction and use its function identifier as name
macro_rules! tx {
    // No params
    ($n:ident) => {
        transaction!($n).set_name(stringify!($n))
    };

    // 1 param, auto display (redirects to custom display with auto-generated name)
    ($n:ident($v1:expr)) => {{
        tx!($n($v1), name: &format!("{}[{}]", stringify!($n), utils::truncate_middle($v1, MAX_ID_DISPLAY)))
    }};

    // 1 param, custom display
    ($n:ident($v1:expr), name: $display:expr) => {{
        let v1 = ($v1).clone();
        let display = $display;
        Transaction::new(Arc::new({
            let v1 = v1.clone();
            move |s| Box::pin($n(v1.clone(), s))
        }))
        .set_name(&display)
    }};

    // 2 params, auto display
    ($n:ident($v1:expr, $v2:expr)) => {{
        tx!($n($v1, $v2), name: &format!("{}[{}]", stringify!($n), utils::truncate_middle($v1, MAX_ID_DISPLAY)))
    }};

    // 2 params, custom display
    ($n:ident($v1:expr, $v2:expr), name: $display:expr) => {{
        let v1 = ($v1).clone();
        let v2 = ($v2).clone();
        let display = $display;
        Transaction::new(Arc::new({
            let v1 = v1.clone();
            let v2 = v2.clone();
            move |s| Box::pin($n(v1.clone(), v2.clone(), s))
        }))
        .set_name(&display)
    }};

    // Optional, 1 param, auto display
    ($s:ident.$n:ident?($v1:expr)) => {
        if let Some(value) = ($v1).clone() {
            $s = $s.register_transaction(tx!($n(value.clone())));
        }
    };

    // Optional, 1 param, custom display
    ($s:ident.$n:ident?($v1:expr), name: $display:expr) => {
        if let Some(value) = ($v1).clone() {
            $s = $s.register_transaction(tx!($n(value.clone()), name: $display));
        }
    };

    // Optional, 2 params, auto display
    ($s:ident.$n:ident?($v1:expr, $v2:expr)) => {
        if let Some(value) = ($v1).clone() {
            $s = $s.register_transaction(tx!($n(value.clone(), $v2)));
        }
    };

    // Optional, 2 params, custom display
    ($s:ident.$n:ident?($v1:expr, $v2:expr), name: $display:expr) => {
        if let Some(value) = ($v1).clone() {
            $s = $s.register_transaction(tx!($n(value.clone(), $v2), name: $display));
        }
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

    let scenario_file = std::env::var("SCENARIO_FILE").ok();

    if !matches!(
        std::env::var("GENERATE_SCENARIO").ok().as_deref(),
        None | Some("false" | "0")
    ) {
        let scenario = scenario::Scenario::eval().await?;
        println!("{}", serde_json::to_string_pretty(&scenario)?);
        return Ok(());
    }

    let scenario = Arc::new(scenario::Scenario::load(scenario_file.as_deref()).await?);

    let custom_client = if matches!(
        std::env::var("AUTH_DISABLED").ok().as_deref(),
        Some("true" | "1")
    ) {
        None
    } else {
        let provider = create_oidc_provider().await?;
        Some(Transaction::new(Arc::new(move |user| {
            let provider = provider.clone();
            Box::pin(async move { setup_custom_client(&provider, user).await })
        })))
    };

    // Create atomic counter for sequential delete strategy
    let delete_counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

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
        .register_scenario({
            create_scenario(
                "WebsiteUser",
                wait_time_from,
                wait_time_to,
                custom_client.clone(),
            )?
            .register_transaction(tx!(website_index))
            .register_transaction(tx!(website_openapi))
            .register_transaction(tx!(website_sboms))
            .register_transaction(tx!(website_packages))
            .register_transaction(tx!(website_advisories))
            .register_transaction(tx!(website_importers))
        })
        .register_scenario({
            let mut s = create_scenario(
                "RestAPIUser",
                wait_time_from,
                wait_time_to,
                custom_client.clone(),
            )?
            .set_weight(5)?
            .register_transaction(list_tx("/api/v2/organization", ""))
            .register_transaction(list_tx("/api/v2/advisory", ""))
            .register_transaction(list_tx("/api/v2/advisory", "offset=100&limit=10"))
            .register_transaction(list_tx("/api/v2/advisory", "q=identifier%3dCVE-2022-0981"))
            .register_transaction(list_tx("/api/v2/advisory", "q=CVE-2021-"))
            .register_transaction(list_tx("/api/v2/vulnerability", ""))
            .register_transaction(list_tx("/api/v2/vulnerability", "offset=100&limit=10"))
            .register_transaction(list_tx("/api/v2/importer", ""))
            .register_transaction(list_tx("/api/v2/purl", ""))
            .register_transaction(list_tx("/api/v2/purl", "offset=100&limit=10"))
            .register_transaction(list_tx("/api/v2/purl", "q=curl"))
            .register_transaction(list_tx("/api/v2/purl", "q=name=curl"))
            .register_transaction(list_tx("/api/v2/product", ""))
            .register_transaction(list_tx("/api/v3/sbom", ""))
            .register_transaction(list_tx("/api/v2/sbom", ""))
            .register_transaction(list_tx("/api/v3/sbom", "offset=100&limit=10"))
            .register_transaction(list_tx("/api/v2/sbom", "offset=100&limit=10"))
            .register_transaction(tx!(list_advisory_labels))
            .register_transaction(list_tx("/api/v2/sbom-labels", ""))
            .register_transaction(list_tx("/api/v2/purl/base", ""))
            .register_transaction(list_tx("/api/v2/license", ""))
            .register_transaction(list_tx("/api/v2/license/spdx/license", ""))
            .register_transaction(list_tx("/api/v2/weakness", ""))
            .register_transaction(list_tx("/api/v2/group/sbom", ""))
            .register_transaction(tx!(post_vulnerability_analyze_v3))
            .register_transaction(list_tx("/.well-known/trustify", ""))
            .register_transaction(tx!(post_extract_sbom_purls))
            .register_transaction(list_tx("/api/v2/advisory", "q=title~openssl"))
            .register_transaction(list_tx("/api/v2/advisory", "q=modified>3 days ago"))
            .register_transaction(list_tx("/api/v2/advisory", "sort=modified:desc"))
            .register_transaction(list_tx("/api/v2/advisory", "deprecated=Consider"))
            .register_transaction(list_tx("/api/v3/sbom", "q=name~redhat"))
            .register_transaction(list_tx("/api/v3/sbom", "q=published>2024-01-01"))
            .register_transaction(list_tx("/api/v3/sbom", "sort=ingested:desc"))
            .register_transaction(list_tx("/api/v3/sbom", "q=label:type=product"))
            .register_transaction(list_tx("/api/v2/vulnerability", "q=base_severity=high"))
            .register_transaction(list_tx("/api/v2/vulnerability", "q=base_score>=7.0"))
            .register_transaction(list_tx("/api/v2/vulnerability", "q=cwes=CWE-79"))
            .register_transaction(list_tx("/api/v2/vulnerability", "sort=base_score:desc"))
            .register_transaction(list_tx("/api/v2/purl", "q=purl:ty=rpm"))
            .register_transaction(list_tx("/api/v2/purl", "q=purl:namespace=redhat"))
            .register_transaction(list_tx("/api/v2/purl", "sort=purl:name:asc"))
            .register_transaction(list_tx("/api/v2/purl/base", "q=type=rpm"))
            .register_transaction(list_tx("/api/v2/purl/base", "q=namespace=redhat"))
            .register_transaction(list_tx("/api/v2/purl/base", "sort=name:asc"))
            .register_transaction(list_tx("/api/v2/organization", "sort=name:asc"))
            .register_transaction(list_tx("/api/v2/product", "q=name~openshift"))
            .register_transaction(list_tx("/api/v2/product", "sort=name:asc"))
            .register_transaction(list_tx("/api/v2/weakness", "q=description~injection"))
            .register_transaction(list_tx("/api/v2/weakness", "sort=id:asc"))
            .register_transaction(list_tx("/api/v2/group/sbom", "totals=true"))
            .register_transaction(list_tx("/api/v2/group/sbom", "parents=resolve"));

            tx!(s.get_sbom?(scenario.get_sbom.clone()));
            tx!(s.get_sbom_advisories?(scenario.get_sbom_advisories.clone()));
            tx!(s.get_sbom_packages?(scenario.get_sbom_packages.clone()));
            tx!(s.get_sbom_related?(scenario.get_sbom_related.clone()));
            tx!(s.get_vulnerability?(scenario.get_vulnerability.clone()));
            tx!(s.sbom_by_package?(scenario.sbom_by_package.clone()));
            tx!(s.get_sbom_license_ids?(scenario.sbom_license_ids.clone()));
            tx!(s.post_vulnerability_analyze?(scenario.analyze_purl.clone()));
            tx!(s.get_purl_details?(scenario.get_purl_details.clone()));
            tx!(s.get_recommendations?(scenario.get_recommendations.clone()));

            tx!(s.download_advisory?(scenario.download_advisory.clone()));
            tx!(s.get_advisory?(scenario.get_advisory.clone()));

            tx!(s.download_sbom?(scenario.download_sbom.clone()));
            tx!(s.get_sbom_license_export?(
                scenario.get_sbom_license_export.clone()
            ));
            tx!(s.count_sbom_by_package?(
                scenario.count_sbom_by_package.clone()
            ));
            tx!(s.get_sbom_group?(scenario.get_sbom_group.clone()));
            tx!(s.get_sbom_group_assignments?(
                scenario.get_sbom_group.clone()
            ));
            tx!(s.get_product?(scenario.get_product.clone()));
            tx!(s.get_organization?(scenario.get_organization.clone()));
            tx!(s.get_base_purl?(scenario.get_base_purl.clone()));
            tx!(s.get_importer?(scenario.get_importer.clone()));
            tx!(s.get_importer_report?(scenario.get_importer.clone()));
            tx!(s.get_weakness?(scenario.get_weakness.clone()));
            tx!(s.get_spdx_license?(scenario.get_spdx_license.clone()));
            s
        })
        .register_scenario({
            create_scenario(
                "RestAPIUserSlow",
                wait_time_from,
                wait_time_to,
                custom_client.clone(),
            )?
            .set_weight(1)?
            .register_transaction(list_tx("/api/v2/license", "q=ASL&sort=license:desc"))
            .register_transaction(list_tx("/api/v2/sbom", "q=license~GPL&sort=name:desc"))
            .register_transaction(list_tx("/api/v2/purl", "q=license~GPLv3+ with exceptions|Apache&sort=name:desc"))
            .register_transaction(list_tx("/api/v2/license", "q=license~Apache"))
            .register_transaction(list_tx("/api/v2/license", "q=license~GPL"))
            .register_transaction(list_tx("/api/v2/license/spdx/license", "q=apache"))
            .register_transaction(list_tx("/api/v2/license/spdx/license", "q=gpl"))
        })
        .register_scenario({
            let mut s = create_scenario(
                "AnalysisUser",
                wait_time_from,
                wait_time_to,
                custom_client.clone(),
            )?
            .set_weight(2)?
            .register_transaction(list_tx("/api/v2/analysis/status", ""))
            .register_transaction(list_tx("/api/v2/analysis/latest/component/cpe%3A%2Fa%3Aredhat%3Aopenshift_builds%3A1.3%3A%3Ael9", ""))
            // TODO: .register_transaction(tx!(search_analysis_component))
            // TODO: .register_transaction(tx!(search_latest_component))
            // TODO: .register_transaction(search_tx("/api/v2/analysis/component","q=openssl&descendants=1"))
            // TODO: .register_transaction(search_tx("/api/v2/analysis/component", "q=curl&relationships=contains,dependency"))
            ;

            tx!(s.get_analysis_component?(
                scenario.get_analysis_component.clone()
            ));
            tx!(s.render_sbom_graph_dot?(scenario.render_sbom_graph.clone()));

            s
        })
        .register_scenario({
            let mut s = create_scenario(
                "RestAPIUserDelete",
                wait_time_from,
                wait_time_to,
                custom_client.clone(),
            )?
            .set_weight(1)?
            // With 100 SBOM IDs this ensure they all delete something in the sequential situation
            .set_wait_time(Duration::from_secs(3), Duration::from_secs(4))?;
            // Register delete transaction if pool is available
            if let Some(pool) = scenario.delete_sbom_pool.clone() {
                tx!(s.delete_sbom_from_pool_sequential?(
                    scenario.delete_sbom_pool.clone(),
                    delete_counter.clone()
                ),
                name: format! ("delete_sbom_from_pool_sequential[{} SBOMs]", pool.len()))
            }
            s
        })
        .register_scenario({
            let mut s = create_scenario(
                "RestSBOMLabelUser",
                wait_time_from,
                wait_time_to,
                custom_client.clone(),
            )?
            .set_weight(2)?;
            tx!(s.put_sbom_labels?(scenario.get_sbom_license_export.clone()));
            tx!(s.patch_sbom_labels?(
                scenario.get_sbom_license_export.clone()
            ));
            s
        })
        .register_scenario({
            let mut s = create_scenario(
                "RestAdvisoryLableUser",
                wait_time_from,
                wait_time_to,
                custom_client,
            )?
            .set_weight(5)?;
            // Register advisory label transactions if host is available.
            // Since the scenario object doesn't provide host information, we use environment
            let host = s
                .host
                .clone()
                .or_else(|| std::env::var("HOST").ok())
                .unwrap_or_else(|| "http://localhost:8080".to_string());
            let total_advisories = get_advisory_total(host).await.ok();
            if let Some(total) = total_advisories {
                tx!(s.find_random_advisory?(Some(total)));
                s = s.register_transaction(tx!(put_advisory_labels));
                s = s.register_transaction(tx!(patch_advisory_labels));
            }
            s
        })
        .execute()
        .await?;

    Ok(())
}

fn create_scenario(
    name: &str,
    wait_time_from: u64,
    wait_time_to: u64,
    custom_client: Option<Transaction>,
) -> Result<Scenario, GooseError> {
    let mut s = scenario!(name);
    if let Some(client) = custom_client {
        s = s.register_transaction(client.set_name("logon"));
    }
    s.set_wait_time(
        Duration::from_secs(wait_time_from),
        Duration::from_secs(wait_time_to),
    )
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
    use reqwest::header;

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
    let builder = reqwest_12::Client::builder()
        .default_headers(headers)
        .user_agent("loadtest-ua")
        .timeout(timeout);

    // Assign the custom client to this GooseUser.
    user.set_client_builder(builder).await?;

    Ok(())
}
