use anyhow::{Context, Error};
use shuttle_runtime::SecretStore;

#[derive(Clone)]
pub struct Config {
    pub swagger_uri: url::Url,
    pub database_url: url::Url,
}

impl Config {
    pub fn new(secrets: SecretStore) -> Result<Self, Error> {
        let swagger_uri = secrets
            .get("SWAGGER_URI")
            .context("SWAGGER_URI is required")?;
        let database_url = secrets
            .get("DATABASE_URL")
            .context("DATABASE_URL is required")?;

        let swagger_uri = url::Url::parse(&swagger_uri).context("Cannot parse swagger uri")?;
        let database_url = url::Url::parse(&database_url).context("Cannot parse database uri")?;

        Ok(Self {
            swagger_uri,
            database_url,
        })
    }
}
