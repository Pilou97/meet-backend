use anyhow::{Context, Error};
use shuttle_runtime::SecretStore;

use crate::ports::output::config::Config;

pub struct ShuttleConfig {
    swagger_uri: url::Url,
    livekit_api_key: String,
    livekit_secret: String,
}

impl ShuttleConfig {
    pub fn new(secrets: SecretStore) -> Result<Self, Error> {
        let swagger_uri = secrets
            .get("SWAGGER_URI")
            .context("SWAGGER_URI is required")?;

        let livekit_api_key = secrets
            .get("LIVEKIT_API_KEY")
            .context("LIVEKIT_API_KEY is required")?;

        let livekit_secret = secrets
            .get("LIVEKIT_SECRET")
            .context("LIVEKIT_SECRET is required")?;

        let swagger_uri = url::Url::parse(&swagger_uri).context("Cannot parse swagger uri")?;

        Ok(Self {
            swagger_uri,
            livekit_api_key,
            livekit_secret,
        })
    }
}

impl Config for ShuttleConfig {
    fn swagger_uri(&self) -> url::Url {
        self.swagger_uri.clone()
    }

    fn livekit_api_key(&self) -> String {
        self.livekit_api_key.clone()
    }

    fn livekit_secret(&self) -> String {
        self.livekit_secret.clone()
    }
}
