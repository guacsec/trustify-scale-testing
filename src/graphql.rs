use goose::goose::{GooseUser, TransactionResult};

pub async fn graphql_query_advisory(user: &mut GooseUser) -> TransactionResult {
    let body = r#"{"query": "{getAdvisories { id sha256 published organization { id website } vulnerabilities { id title }}}" }"#;
    let _response = user.post("/graphql", body).await?;
    Ok(())
}
