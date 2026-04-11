use crate::utils::DisplayVec;
use goose::goose::{GooseUser, TransactionResult};
use serde_json::json;
use urlencoding::encode;

pub async fn get_purl_details(purl_id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v3/purl/{purl_id}")).await?;

    Ok(())
}

pub async fn get_base_purl(key: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/purl/base/{}", encode(&key)))
        .await?;

    Ok(())
}

/// Send a recommend request with a subset of PURLs determined by `batch_size`.
pub async fn get_recommendations(
    purls: DisplayVec<String>,
    batch_size: usize,
    user: &mut GooseUser,
) -> TransactionResult {
    let batch: Vec<&String> = purls.0.iter().take(batch_size).collect();
    let _response = user
        .post_json(
            "/api/v2/purl/recommend",
            &json!({
             "purls": batch
            }),
        )
        .await?;
    Ok(())
}
