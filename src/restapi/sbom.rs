use goose::goose::{GooseUser, TransactionResult};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use urlencoding::encode;

pub async fn list_sboms(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v3/sbom").await?;

    Ok(())
}

pub async fn list_sboms_v2(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/sbom").await?;

    Ok(())
}

pub async fn list_sboms_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v3/sbom?offset=100&limit=10").await?;

    Ok(())
}

pub async fn list_sboms_paginated_v2(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/sbom?offset=100&limit=10").await?;

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

pub async fn search_sboms_by_license(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/sbom?q=license~GPL&sort=name:desc")
        .await?;
    Ok(())
}

pub async fn delete_sbom_from_pool_sequential(
    pool: Vec<String>,
    counter: Arc<AtomicUsize>,
    user: &mut GooseUser,
) -> TransactionResult {
    let index = counter.fetch_add(1, Ordering::Relaxed);
    if index < pool.len() {
        let sbom_id = &pool[index];
        let _response = user.delete(&format!("/api/v2/sbom/{sbom_id}")).await?;
    }
    Ok(())
}
