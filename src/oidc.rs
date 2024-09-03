use anyhow::Context;
use core::fmt;
use openid::TemporalBearerGuard;
use reqwest::Url;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, PartialEq, Eq, clap::Args)]
#[command(next_help_heading = "OIDC client configuration")]
pub struct OpenIdTokenProviderConfigArguments {
    #[arg(
        id = "oidc_client_id",
        long = "oidc-client-id",
        env = "OIDC_PROVIDER_CLIENT_ID"
    )]
    pub client_id: String,
    #[arg(
        id = "oidc_client_secret",
        long = "oidc-client-secret",
        env = "OIDC_PROVIDER_CLIENT_SECRET"
    )]
    pub client_secret: String,
    #[arg(
        id = "oidc_issuer_url",
        long = "oidc-issuer-url",
        env = "OIDC_PROVIDER_ISSUER_URL"
    )]
    pub issuer_url: String,
    #[arg(
        id = "oidc_refresh_before",
        long = "oidc-refresh-before",
        env = "OIDC_PROVIDER_REFRESH_BEFORE",
        default_value = "30s"
    )]
    pub refresh_before: humantime::Duration,
    /// Use insecure TLS when contacting the OIDC issuer
    #[arg(
        id = "oidc_insecure_tls",
        long = "oidc-insecure-tls",
        env = "OIDC_PROVIDER_TLS_INSECURE",
        default_value = "false"
    )]
    pub tls_insecure: bool,
}

/// A provider which provides access tokens for clients.
#[derive(Clone)]
pub struct OpenIdTokenProvider {
    client: Arc<::openid::Client>,
    current_token: Arc<RwLock<Option<openid::TemporalBearerGuard>>>,
    refresh_before: chrono::Duration,
}

impl Debug for OpenIdTokenProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenProvider")
            .field(
                "client",
                &format!("{} / {:?}", self.client.client_id, self.client.http_client),
            )
            .field("current_token", &"...")
            .finish()
    }
}

impl OpenIdTokenProvider {
    /// Create a new provider using the provided client.
    pub fn new(client: openid::Client, refresh_before: chrono::Duration) -> Self {
        Self {
            client: Arc::new(client),
            current_token: Arc::new(RwLock::new(None)),
            refresh_before,
        }
    }

    pub async fn with_config(config: OpenIdTokenProviderConfigArguments) -> anyhow::Result<Self> {
        let issuer = Url::parse(&config.issuer_url).context("Parse issuer URL")?;
        let mut client = reqwest::ClientBuilder::new();

        if config.tls_insecure {
            log::warn!("Using insecure TLS when contacting the OIDC issuer");
            client = client
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true);
        }

        let client = openid::Client::discover_with_client(
            client.build()?,
            config.client_id,
            config.client_secret,
            None,
            issuer,
        )
        .await
        .context("Discover OIDC client")?;
        Ok(Self::new(
            client,
            chrono::Duration::from_std(config.refresh_before.into())?,
        ))
    }

    /// return a fresh token, this may be an existing (non-expired) token
    /// a newly refreshed token.
    pub async fn provide_token(&self) -> Result<openid::Bearer, openid::error::Error> {
        match self.current_token.read().await.deref() {
            Some(token) if !token.expires_before(self.refresh_before) => {
                log::debug!("Token still valid");
                return Ok(token.as_ref().clone());
            }
            _ => {}
        }

        // fetch fresh token after releasing the read lock

        self.fetch_fresh_token().await
    }

    async fn fetch_fresh_token(&self) -> Result<openid::Bearer, openid::error::Error> {
        log::debug!("Fetching fresh token...");

        let mut lock = self.current_token.write().await;

        match lock.deref() {
            // check if someone else refreshed the token in the meantime
            Some(token) if !token.expires_before(self.refresh_before) => {
                log::debug!("Token already got refreshed");
                return Ok(token.as_ref().clone());
            }
            _ => {}
        }

        // we hold the write-lock now, and can perform the refresh operation

        let next_token = match lock.take() {
            // if we don't have any token, fetch an initial one
            None => {
                log::debug!("Fetching initial token... ");
                self.initial_token().await?
            }
            // if we have an expired one, refresh it
            Some(current_token) => {
                log::debug!("Refreshing token ... ");
                match current_token.as_ref().refresh_token.is_some() {
                    true => self.client.refresh_token(current_token, None).await?.into(),
                    false => self.initial_token().await?,
                }
            }
        };

        log::debug!("Next token: {:?}", next_token.as_ref());

        let result = next_token.as_ref().clone();
        lock.replace(next_token);

        // done

        Ok(result)
    }

    async fn initial_token(&self) -> Result<openid::TemporalBearerGuard, openid::error::Error> {
        Ok(self
            .client
            .request_token_using_client_credentials(None)
            .await?
            .into())
    }
}

trait Expires {
    fn expires_before(&self, duration: chrono::Duration) -> bool;
}

impl Expires for TemporalBearerGuard {
    fn expires_before(&self, duration: chrono::Duration) -> bool {
        match self.expires_at() {
            Some(expires) => expires - chrono::Utc::now() <= duration,
            None => false,
        }
    }
}
