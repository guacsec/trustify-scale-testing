use goose::goose::{GooseUser, TransactionResult};

pub async fn list_sbom_groups(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/group/sbom").await?;

    Ok(())
}

pub async fn get_sbom_group(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/group/sbom/{id}")).await?;

    Ok(())
}

pub async fn get_sbom_group_assignments(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/group/sbom-assignment/{id}"))
        .await?;

    Ok(())
}
