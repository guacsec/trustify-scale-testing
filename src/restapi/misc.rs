use goose::goose::{GooseUser, TransactionResult};

pub async fn list_importer(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/importer").await?;

    Ok(())
}

pub async fn list_organizations(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/organization").await?;

    Ok(())
}

pub async fn list_products(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/product").await?;

    Ok(())
}

pub async fn search_licenses(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/license?q=ASL&sort=license:desc").await?;
    Ok(())
}
