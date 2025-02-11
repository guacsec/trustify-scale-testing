// use goose::goose::{GooseUser, TransactionResult};

// pub async fn g_get_advisories(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{"query": "{getAdvisories { id sha256 published organization { id website } vulnerabilities { id title }}}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_get_advisory_by_id(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{ "query": "{ getAdvisoryById(id: \"37292820-00ee-4299-8097-a11f8348bdf8\") { id title }}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_get_organization_by_name(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{"query": "{ getOrganizationByName(name: \"Red Hat\") { id name }}"}"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_get_sbom_by_id(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{ "query": "{ getSbomById(id: \"01926efa-bbb2-7222-9f4d-692bd5e40a46\") {sbomId, authors}}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_get_sbom_by_labels(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{ "query": "{ getSbomsByLabels(labels: \"type:spdx\") { sbomId sourceDocumentId authors labels }}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_cves_by_sbom(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{ "query": "{ cvesBySbom(id: \"01926efa-bbb2-7222-9f4d-692bd5e40a46\") { vulnerabilityId status packages { id name version } }}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_get_vulnerability_by_id(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{ "query": "{ getVulnerabilityById(identifier: \"CVE-2024-28111\") { id published }}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
//
// pub async fn g_get_vulnerabilities(user: &mut GooseUser) -> TransactionResult {
//     let body = r#"{ "query": "{ getVulnerabilities { id published withdrawn }}" }"#;
//     let _response = user.post("/graphql", body).await?;
//     Ok(())
// }
