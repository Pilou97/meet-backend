use adapters::input::http::handlers::meeting::MeetingRouter;
use anyhow::Context;
use poem::{handler, middleware::Cors, EndpointExt, Route};
use shuttle_poem::ShuttlePoem;

pub mod adapters;
pub mod domain;

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

    // used for swagger
    let url = match port {
        Some(port) => format!("{scheme}://{host}:{port}/api"),
        None => format!("{scheme}://{host}/api"),
    };

    let api_service = poem_openapi::OpenApiService::new(MeetingRouter {}, "API", "1.0").server(url);
    let api_swagger = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/ui", api_swagger)
        .nest("openapi.json", spec_json)
        .with(Cors::new());

    Ok(app.into())
}
