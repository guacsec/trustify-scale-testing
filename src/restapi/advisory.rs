use crate::utils::GooseUserData;
use anyhow::Context;
use goose::goose::{GooseMethod, GooseRequest, GooseUser, TransactionError, TransactionResult};
use rand::prelude::*;
use reqwest_12::{Client, RequestBuilder, get};
use serde_json::json;
use urlencoding::encode;

pub async fn get_advisory_total(host: String) -> Result<u64, anyhow::Error> {
    let url = format!("{}/api/v2/advisory", host.trim_end_matches('/'));

    log::info!("Fetching advisory total from: {}", url);

    let response = get(&url)
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

pub async fn find_random_advisory(
    total_advisories: u64,
    user: &mut GooseUser,
) -> TransactionResult {
    let offset = rand::rng().random_range(0..total_advisories);
    let url = format!("/api/v2/advisory?offset={}&limit=1", offset);

    let response = user.get(&url).await?;
    let json_data = response.response?.json::<serde_json::Value>().await?;

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

    Err(Box::new(TransactionError::Custom(format!(
        "No advisory found at offset: {}",
        offset
    ))))
}

pub async fn get_advisory(id: String, user: &mut GooseUser) -> TransactionResult {
    let uri = format!("/api/v2/advisory/{}", encode(&format!("urn:uuid:{}", id)));

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
