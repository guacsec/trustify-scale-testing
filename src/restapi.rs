use goose::goose::{GooseUser, TransactionResult};

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

pub async fn search_packages(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/purl?q=curl").await?;

    Ok(())
}

pub async fn search_exact_packages(user: &mut GooseUser) -> TransactionResult {
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

pub async fn list_analysis_descendants(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/analysis/component?descendants=10")
        .await?;

    Ok(())
}

pub async fn list_analysis_ancestors(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/analysis/component?ancestors=10").await?;

    Ok(())
}

pub async fn list_analysis_search(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/analysis/component?q=lib&descendants=10")
        .await?;

    Ok(())
}
pub async fn list_analysis_paginated(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/analysis/component?descendants=10&offset=1&limit=10")
        .await?;

    Ok(())
}
