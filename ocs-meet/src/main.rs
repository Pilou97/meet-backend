use adapters::output::{livekit::Livekit, repository::db::Repository, shuttle::ShuttleConfig};
use anyhow::Context;
use ports::output::config::Config;
use shuttle_poem::ShuttlePoem;
use sqlx::PgPool;

pub mod adapters;
pub mod app;
pub mod domain;
pub mod ports;
pub mod services;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttlePoem<impl poem::Endpoint> {
    let config = ShuttleConfig::new(secrets)?;

    let repository = Repository::new(pool)
        .await
        .context("Cannot instanciate the repository")?;

    let room_manager = Livekit::new(config.livekit_api_key(), config.livekit_secret());

    let app = app::app(config, repository, room_manager).await?;

    Ok(app.into())
}
