use goose::goose::{GooseUser, TransactionResult};
use serde_json::json;

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

pub async fn get_product(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/product/{id}")).await?;

    Ok(())
}

pub async fn get_organization(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/organization/{id}")).await?;

    Ok(())
}

pub async fn get_importer(name: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/importer/{name}")).await?;

    Ok(())
}

pub async fn get_importer_report(name: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/importer/{name}/report")).await?;

    Ok(())
}

pub async fn list_licenses(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/license").await?;

    Ok(())
}

pub async fn list_spdx_licenses(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/license/spdx/license").await?;

    Ok(())
}

pub async fn get_spdx_license(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .get(&format!("/api/v2/license/spdx/license/{id}"))
        .await?;

    Ok(())
}

pub async fn list_weaknesses(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/api/v2/weakness").await?;

    Ok(())
}

pub async fn get_weakness(id: String, user: &mut GooseUser) -> TransactionResult {
    let _response = user.get(&format!("/api/v2/weakness/{id}")).await?;

    Ok(())
}

pub async fn get_system_info(user: &mut GooseUser) -> TransactionResult {
    let _response = user.get("/.well-known/trustify").await?;

    Ok(())
}

pub async fn post_extract_sbom_purls(user: &mut GooseUser) -> TransactionResult {
    let _response = user
        .post_json(
            "/api/v2/ui/extract-sbom-purls",
            &json!({
                "spdxVersion": "SPDX-2.3",
                "dataLicense": "CC0-1.0",
                "SPDXID": "SPDXRef-DOCUMENT",
                "name": "load-test-doc",
                "documentNamespace": "https://example.com/load-test",
                "packages": [
                    {
                        "SPDXID": "SPDXRef-Package",
                        "name": "openssl",
                        "versionInfo": "3.0.0",
                        "downloadLocation": "NOASSERTION",
                        "filesAnalyzed": false,
                        "externalRefs": [
                            {
                                "referenceCategory": "PACKAGE-MANAGER",
                                "referenceType": "purl",
                                "referenceLocator": "pkg:rpm/redhat/openssl@3.0.0"
                            }
                        ]
                    }
                ],
                "relationships": [
                    {
                        "spdxElementId": "SPDXRef-DOCUMENT",
                        "relationshipType": "DESCRIBES",
                        "relatedSpdxElement": "SPDXRef-Package"
                    }
                ]
            }),
        )
        .await?;

    Ok(())
}
