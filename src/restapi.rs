use goose::goose::{GooseUser, TransactionError, TransactionResult};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use urlencoding::encode;

use crate::utils::DisplayVec;

/// Upload file and return advisory ID
pub async fn upload_advisory_and_get_id(
    advisory_file: String,
    user: &mut GooseUser,
) -> Result<String, Box<TransactionError>> {
    let file_bytes = tokio::fs::read(&advisory_file).await.map_err(|e| {
        Box::new(TransactionError::Custom(format!(
            "Failed to read file {}: {}",
            advisory_file, e
        )))
    })?;

    let response = user.post("/api/v2/advisory", file_bytes).await?;
    let v = response.response?.json::<serde_json::Value>().await?;

    let advisory_id = v["id"]
        .as_str()
        .ok_or_else(|| {
            Box::new(TransactionError::Custom(
                "Missing advisory ID in response".to_string(),
            ))
        })?
        .to_string();

    Ok(advisory_id)
}

/// Delete advisory by ID
pub async fn delete_advisory_by_id(advisory_id: String, user: &mut GooseUser) -> TransactionResult {
    let uri = format!("/api/v2/advisory/{}", advisory_id);
    user.delete(&uri).await?;
    Ok(())
}

/// Sequential execution: upload and then immediately delete
pub async fn upload_and_immediately_delete(
    advisory_file: String,
    user: &mut GooseUser,
) -> TransactionResult {
    // 1. Upload file and get ID
    let advisory_id = upload_advisory_and_get_id(advisory_file, user).await?;

    // 2. Immediately delete (no waiting required)
    delete_advisory_by_id(advisory_id, user).await?;

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
