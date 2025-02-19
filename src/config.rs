use anyhow::{Context, Error};
use shuttle_runtime::SecretStore;

#[derive(Clone)]
pub struct Config {
    pub swagger_uri: url::Url,
    pub livekit_api_key: String,
    pub livekit_secret: String,
}

impl Config {
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
