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

    #[cfg(test)]
    pub fn create_mock() -> Self {
        use shuttle_common::secrets::Secret;
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert(
            "SWAGGER_URI".to_string(),
            Secret::from("http://localhost:8000".to_string()),
        );
        map.insert(
            "DATABASE_URL".to_string(),
            Secret::from("postgres://test@test.com".to_string()),
        );
        map.insert(
            "LIVEKIT_API_KEY".to_string(),
            Secret::from("livekit_api_key".to_string()),
        );
        map.insert(
            "LIVEKIT_SECRET".to_string(),
            Secret::from("livekit_secret".to_string()),
        );
        let secrets = SecretStore::new(map);
        Config::new(secrets).unwrap()
    }
}
