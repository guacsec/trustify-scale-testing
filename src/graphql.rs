use goose::goose::{GooseUser, TransactionResult};
use urlencoding::encode;

pub async fn graphql_query_advisory(user: &mut GooseUser) -> TransactionResult {
    let query = r#"{"query": "{getAdvisories { id sha256 published organization { id website } vulnerabilities { id title }}}" }"#;
    let encoded_query = encode(query);
    let url = format!("/graphql?query={}", encoded_query);
    let _response = user.get(&url).await?;
    Ok(())
}
