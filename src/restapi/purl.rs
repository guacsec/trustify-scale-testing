use crate::utils::DisplayVec;
use goose::goose::{GooseUser, TransactionResult};
use serde_json::json;
use urlencoding::encode;

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

pub async fn search_purls_by_license(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/purl?q=license~GPLv3+ with exceptions|Apache&sort=name:desc")
        .await?;
    Ok(())
}

pub async fn list_base_purls(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/purl/base").await?;

    Ok(())
}

pub async fn get_base_purl(key: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/purl/base/{}", encode(&key)))
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
