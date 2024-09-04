use goose::goose::{GooseUser, TransactionResult};

pub async fn list_advisory(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/advisory").await?;

    Ok(())
}

pub async fn list_advisory_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/advisory?offset=100&limit=10").await?;

    Ok(())
}

pub async fn get_advisory_by_doc_id(user: &mut GooseUser) -> TransactionResult {
    // just pick one CVE that should be in the dump: CVE-2022-0981
    let _response = user
        .get("/api/v1/advisory?q=identifier%3dCVE-2022-0981")
        .await?;

    Ok(())
}

pub async fn list_importer(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/importer").await?;

    Ok(())
}

pub async fn list_organizations(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/organization").await?;

    Ok(())
}

pub async fn list_packages(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/purl").await?;

    Ok(())
}

pub async fn list_packages_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/purl?offset=100&limit=10").await?;

    Ok(())
}

pub async fn search_packages(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/purl?q=curl").await?;

    Ok(())
}

pub async fn list_products(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/product").await?;

    Ok(())
}

pub async fn list_sboms(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/sbom").await?;

    Ok(())
}

pub async fn list_sboms_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/sbom?offset=100&limit=10").await?;

    Ok(())
}

pub async fn list_vulnerabilities(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/vulnerability").await?;

    Ok(())
}

pub async fn list_vulnerabilities_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v1/vulnerability?offset=100&limit=10")
        .await?;

    Ok(())
}
