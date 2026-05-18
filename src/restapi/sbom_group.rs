use goose::goose::{GooseUser, TransactionResult};

pub async fn get_sbom_group(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v3/group/sbom/{id}")).await?;

    Ok(())
}

pub async fn get_sbom_group_assignments(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v3/group/sbom-assignment/{id}"))
        .await?;

    Ok(())
}
