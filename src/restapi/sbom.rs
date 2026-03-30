use goose::goose::{GooseMethod, GooseRequest, GooseUser, TransactionResult};
use reqwest_12::Client;
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use urlencoding::encode;

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

pub async fn download_sbom(key: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/sbom/{}/download", encode(&key)))
        .await?;

    Ok(())
}

pub async fn get_sbom_license_export(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/sbom/{}/license-export", encode(&id)))
        .await?;

    Ok(())
}

pub async fn count_sbom_by_package(purl: String, user: &mut GooseUser) -> TransactionResult {
    let url = user.build_url("/api/v2/sbom/count-by-package")?;
    let goose_request = GooseRequest::builder()
        .method(GooseMethod::Get)
        .path("/api/v2/sbom/count-by-package")
        .set_request_builder(Client::get(&user.client, url).json(&json!([{"purl": purl}])))
        .build();
    let _response = user.request(goose_request).await?;

    Ok(())
}

pub async fn put_sbom_labels(id: String, user: &mut GooseUser) -> TransactionResult {
    send_sbom_label_request(id, user, GooseMethod::Put, "load-test", Client::put).await
}

pub async fn patch_sbom_labels(id: String, user: &mut GooseUser) -> TransactionResult {
    send_sbom_label_request(
        id,
        user,
        GooseMethod::Patch,
        "load-test-patch",
        Client::patch,
    )
    .await
}

async fn send_sbom_label_request(
    sbom_id: String,
    user: &mut GooseUser,
    method: GooseMethod,
    source: &str,
    client_method: fn(&Client, String) -> reqwest_12::RequestBuilder,
) -> TransactionResult {
    use serde_json::json;
    let path = format!("/api/v2/sbom/{}/label", encode(&sbom_id));
    let json = json!({
        "source": source,
        "load-test": "true",
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
