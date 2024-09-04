use goose::goose::{GooseUser, TransactionResult};

pub async fn get_advisory(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/advisory").await?;

    Ok(())
}

pub async fn get_importer(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/importer").await?;

    Ok(())
}

pub async fn get_organizations(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/organization").await?;

    Ok(())
}

pub async fn get_packages(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/purl").await?;

    Ok(())
}

pub async fn search_packages(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/purl?q=curl").await?;

    Ok(())
}

pub async fn get_products(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/product").await?;

    Ok(())
}

pub async fn get_sboms(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/sbom").await?;

    Ok(())
}

pub async fn get_vulnerabilities(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v1/vulnerability").await?;

    Ok(())
}
