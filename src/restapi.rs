use crate::utils::{DisplayVec, GooseUserData, generate_advisory_content_async};
use anyhow::Context;
use goose::goose::{GooseMethod, GooseRequest, GooseUser, TransactionError, TransactionResult};
use rand::Rng;
use reqwest::{Client, RequestBuilder};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use urlencoding::encode;

/// Fetch advisory total count once at application startup
pub async fn get_advisory_total(host: String) -> Result<u64, anyhow::Error> {
    let url = format!("{}/api/v2/advisory", host.trim_end_matches('/'));

    log::info!("Fetching advisory total from: {}", url);

    let response = reqwest::get(&url)
        .await
        .context("Failed to send request to get advisory total")?
        .error_for_status()
        .context("Failed to get advisory total")?;

    let json_data = response.json::<serde_json::Value>().await?;

    if let Some(total) = json_data.get("total").and_then(|t| t.as_u64()) {
        return Ok(total);
    }
    Err(anyhow::anyhow!(
        "Failed to get advisory total count".to_string(),
    ))
}

/// Get a random advisory ID
pub async fn find_random_advisory(
    total_advisories: u64,
    user: &mut GooseUser,
) -> TransactionResult {
    // Generate random offset using the provided total
    let offset = rand::rng().random_range(0..=total_advisories);
    let url = format!("/api/v2/advisory?offset={}&limit=1", offset);

    let response = user.get(&url).await?;
    let json_data = response.response?.json::<serde_json::Value>().await?;

    // Extract advisory ID from the response
    if let Some(items) = json_data.get("items").and_then(|i| i.as_array())
        && let Some(first_item) = items.first()
        && let Some(id) = first_item.get("uuid").and_then(|u| u.as_str())
    {
        log::info!("Listing advisory with offset {}: {}", offset, id);

        user.set_session_data(GooseUserData {
            advisory_id: Some(id.to_string()),
        });
        return Ok(());
    }

    // Return error if no advisory found
    Err(Box::new(TransactionError::Custom(format!(
        "No advisory found at offset: {}",
        offset
    ))))
}

/// Upload advisory data and extract advisory ID from response
pub async fn upload_advisory(file_bytes: Vec<u8>, user: &mut GooseUser) -> TransactionResult {
    let generate_advisory_str = generate_advisory_content_async(file_bytes).await?;
    let response = user.post("/api/v2/advisory", generate_advisory_str).await?;
    let v = response.response?.json::<serde_json::Value>().await?;

    if let Some(id_str) = v["id"].as_str() {
        let advisory_id = id_str.to_string();
        user.set_session_data(GooseUserData {
            advisory_id: Some(advisory_id.clone()),
        });
    }

    Ok(())
}

pub async fn get_advisory(id: String, user: &mut GooseUser) -> TransactionResult {
    let uri: String = format!("/api/v2/advisory/{}", encode(&format!("urn:uuid:{}", id)));

    let _response = user.get(&uri).await?;

    Ok(())
}

pub async fn download_advisory(id: String, user: &mut GooseUser) -> TransactionResult {
    let uri = format!(
        "/api/v2/advisory/{}/download",
        encode(&format!("urn:uuid:{}", id))
    );

    let _response = user.get(&uri).await?;

    Ok(())
}

pub async fn list_advisory_labels(user: &mut GooseUser) -> TransactionResult {
    let uri = format!(
        "/api/v2/advisory-labels?filter_text={}&limit={}",
        encode("type"),
        1000
    );

    let _response = user.get(&uri).await?;

    Ok(())
}

/// Delete advisory by ID
pub async fn delete_advisory(user: &mut GooseUser) -> TransactionResult {
    let advisory_id = get_advisory_id(user)?;
    let uri = format!("/api/v2/advisory/{}", advisory_id);
    let _response = user.delete(&uri).await?;
    Ok(())
}

pub async fn list_advisory(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/advisory").await?;

    Ok(())
}

pub async fn list_advisory_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/advisory?offset=100&limit=10").await?;

    Ok(())
}

pub async fn get_advisory_by_doc_id(user: &mut GooseUser) -> TransactionResult {
    // just pick one CVE that should be in the dump: CVE-2022-0981
    let _response = user
        .get("/api/v2/advisory?q=identifier%3dCVE-2022-0981")
        .await?;

    Ok(())
}

pub async fn search_advisory(user: &mut GooseUser) -> TransactionResult {
    // search for whatever value is fine (e.g. 'this-string-is-not-important') to trigger the load
    // on the search so decided for 'CVE-2021-' that also  represents a potential user search
    let _response = user.get("/api/v2/advisory?q=CVE-2021-").await?;

    Ok(())
}

/// Send Advisory labels request
async fn send_advisory_label_request(
    advisory_id: String,
    user: &mut GooseUser,
    method: GooseMethod,
    source: &str,
    client_method: fn(&Client, String) -> RequestBuilder,
) -> TransactionResult {
    let path = format!("/api/v2/advisory/{}/label", advisory_id);
    let json = json!({
        "source": source,
        "foo": "bar",
        "space": "with space",
        "empty": "",
    });

    let url = user.build_url(&path)?;

    let reqwest_request_builder = client_method(&user.client, url);
    let goose_request = GooseRequest::builder()
        .method(method)
        .path(path.as_str())
        .set_request_builder(reqwest_request_builder.json(&json))
        .build();
    let _response = user.request(goose_request).await?;

    Ok(())
}

/// Get advisory ID from Goose user data
fn get_advisory_id(user: &mut GooseUser) -> Result<String, Box<TransactionError>> {
    let advisory_id = {
        let goose_user_data = user
            .get_session_data_mut::<GooseUserData>()
            .ok_or_else(|| {
                Box::new(TransactionError::Custom(
                    "No GooseUserData found, please initialize user data first".to_string(),
                ))
            })?;

        goose_user_data.advisory_id.clone().ok_or_else(|| {
            Box::new(TransactionError::Custom(
                "No advisory_id found in GooseUserData".to_string(),
            ))
        })?
    };
    Ok(advisory_id)
}

/// Send Advisory labels request using PUT method
pub async fn put_advisory_labels(user: &mut GooseUser) -> TransactionResult {
    let advisory_id = get_advisory_id(user)?;
    send_advisory_label_request(
        advisory_id,
        user,
        GooseMethod::Put,
        "It's a put request",
        Client::put,
    )
    .await
}

/// Send Advisory labels request using PATCH method
pub async fn patch_advisory_labels(user: &mut GooseUser) -> TransactionResult {
    let advisory_id = {
        let goose_user_data = user
            .get_session_data_mut::<GooseUserData>()
            .ok_or_else(|| {
                Box::new(TransactionError::Custom(
                    "No GooseUserData found, please initialize user data first".to_string(),
                ))
            })?;

        goose_user_data.advisory_id.clone().ok_or_else(|| {
            Box::new(TransactionError::Custom(
                "No advisory_id found in GooseUserData".to_string(),
            ))
        })?
    };
    send_advisory_label_request(
        advisory_id,
        user,
        GooseMethod::Patch,
        "It's a patch request",
        Client::patch,
    )
    .await
}

pub async fn list_importer(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/importer").await?;

    Ok(())
}

pub async fn list_organizations(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/organization").await?;

    Ok(())
}

pub async fn list_packages(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/purl").await?;

    Ok(())
}

pub async fn list_packages_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/purl?offset=100&limit=10").await?;

    Ok(())
}

pub async fn get_purl_details(purl_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/purl/{purl_id}")).await?;

    Ok(())
}

pub async fn search_purls(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/purl?q=curl").await?;

    Ok(())
}

pub async fn search_exact_purl(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/purl?q=name=curl").await?;

    Ok(())
}
pub async fn list_products(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/product").await?;

    Ok(())
}

pub async fn list_sboms(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/sbom").await?;

    Ok(())
}

pub async fn list_sboms_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/sbom?offset=100&limit=10").await?;

    Ok(())
}

pub async fn list_vulnerabilities(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/vulnerability").await?;

    Ok(())
}

pub async fn list_vulnerabilities_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/vulnerability?offset=100&limit=10")
        .await?;

    Ok(())
}

pub async fn get_sbom(sbom_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/sbom/{sbom_id}")).await?;

    Ok(())
}

pub async fn get_sbom_advisories(sbom_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/sbom/{sbom_id}/advisory"))
        .await?;

    Ok(())
}

pub async fn get_sbom_packages(sbom_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/sbom/{sbom_id}/packages"))
        .await?;

    Ok(())
}

pub async fn get_sbom_related(sbom_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/sbom/{sbom_id}/related")).await?;

    Ok(())
}

pub async fn get_vulnerability(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/vulnerability/{id}")).await?;

    Ok(())
}

pub async fn sbom_by_package(purl: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/sbom/by-package?purl={}", encode(&purl)))
        .await?;

    Ok(())
}

pub async fn get_sbom_license_ids(sbom_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!(
            "/api/v2/sbom/{}/all-license-ids",
            encode(&sbom_id)
        ))
        .await?;

    Ok(())
}

pub async fn get_analysis_status(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/analysis/status").await?;

    Ok(())
}

pub async fn get_analysis_latest_cpe(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/analysis/latest/component/cpe%3A%2Fa%3Aredhat%3Aopenshift_builds%3A1.3%3A%3Ael9").await?;

    Ok(())
}

pub async fn post_vulnerability_analyze(purl: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .post_json(
            "/api/v2/vulnerability/analyze",
            &json!({
                "purls": [
                     purl
                ]
            }),
        )
        .await?;

    Ok(())
}

pub async fn search_licenses(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/license?q=ASL&sort=license:desc").await?;
    Ok(())
}

pub async fn search_sboms_by_license(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/sbom?q=license~GPL&sort=name:desc")
        .await?;
    Ok(())
}

pub async fn search_purls_by_license(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/purl?q=license~GPLv3+ with exceptions|Apache&sort=name:desc")
        .await?;
    Ok(())
}

pub async fn get_recommendations(
    purls: DisplayVec<String>,
    user: &mut GooseUser,
) -> TransactionResult {
    let _response = user
        .post_json(
            "/api/v2/purl/recommend",
            &json!({
             "purls": purls
            }),
        )
        .await?;
    Ok(())
}

/// Delete an SBOM by ID from a pre-populated pool using sequential iteration
/// Sequentially iterates through the pool using an atomic counter
pub async fn delete_sbom_from_pool_sequential(
    pool: Vec<String>,
    counter: Arc<AtomicUsize>,
    user: &mut GooseUser,
) -> TransactionResult {
    // Get next index atomically and increment, wrapping around pool size
    let index = counter.fetch_add(1, Ordering::Relaxed);
    if index < pool.len() {
        let sbom_id = &pool[index];
        let _response = user.delete(&format!("/api/v2/sbom/{sbom_id}")).await?;
    }
    Ok(())
}
