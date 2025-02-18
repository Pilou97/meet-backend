use adapters::output::repository::repository::Repository;
use anyhow::Context;
use app::app;
use config::Config;
use poem::handler;
use shuttle_poem::ShuttlePoem;

pub mod adapters;
pub mod app;
pub mod config;
pub mod domain;
pub mod ports;

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> ShuttlePoem<impl poem::Endpoint> {
    let config = Config::new(secrets)?;

    let repository = Repository::new(&config.database_url)
        .await
        .context("Cannot instanciate the repository")?;

    let app = app(config, repository).await?;

    Ok(app.into())
}
