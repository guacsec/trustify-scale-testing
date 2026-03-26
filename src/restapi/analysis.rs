use goose::goose::{GooseUser, TransactionResult};
use urlencoding::encode;

pub async fn get_analysis_status(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/analysis/status").await?;

    Ok(())
}

pub async fn get_analysis_latest_cpe(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/analysis/latest/component/cpe%3A%2Fa%3Aredhat%3Aopenshift_builds%3A1.3%3A%3Ael9").await?;

    Ok(())
}

pub async fn search_analysis_component(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/analysis/component?q=openssl").await?;

    Ok(())
}

pub async fn get_analysis_component(key: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/analysis/component/{}", encode(&key)))
        .await?;

    Ok(())
}

pub async fn search_latest_component(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get("/api/v2/analysis/latest/component?q=openssl")
        .await?;

    Ok(())
}

pub async fn render_sbom_graph_dot(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/analysis/sbom/{}/render.dot", encode(&id)))
        .await?;

    Ok(())
}
