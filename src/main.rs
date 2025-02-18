use adapters::output::repository::Repository;
use anyhow::Context;
use app::app;
use poem::handler;
use shuttle_poem::ShuttlePoem;

pub mod adapters;
pub mod app;
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
    let scheme = secrets.get("SCHEME").unwrap_or("http".into());
    let host = secrets
        .get("HOST")
        .context("The HOST variable has to be defined")?;

    let port = {
        let port = secrets.get("PORT");
        match port {
            None => None,
            Some(port) => Some(port.parse::<u16>().context("The PORT is not valid")?),
        }
    };

    let repository = Repository::new("Hello world".into())
        .await
        .context("Cannot instanciate the repository")?;

    let app = app(scheme, host, port, repository).await?;

    Ok(app.into())
}
