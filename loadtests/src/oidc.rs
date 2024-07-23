// oidc-cli wrapper invoke
use log::{info, warn};

use std::process::Command;

pub fn get_token(issuer_url: String, client_id: String, client_secret: String) -> String {
    let create_client = Command::new("oidc")
        .args([
            "create",
            "confidential",
            "--force",
            "testing-client",
            "--issuer",
            &issuer_url,
            "--client-id",
            &client_id,
            "--client-secret",
            &client_secret,
        ])
        .output()
        .expect("Failed to execute command");

    match create_client.status.code() {
        Some(_value) => info!("Client created successfully"),
        None => warn!("Error creating client: {}", create_client.status),
    }

    let fetch_token = Command::new("oidc")
        .args(["token", "testing-client", "--bearer"])
        .output()
        .expect("Failed to execute command");

    let mut auth_token: String = String::new();

    if fetch_token.status.code().is_some() {
        auth_token = String::from_utf8_lossy(&fetch_token.stdout)
            .to_string()
            .replace(['\n', '\r'], "");
    }

    auth_token
}
