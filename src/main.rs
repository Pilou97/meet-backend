use adapters::input::http::handlers::meeting::MeetingRouter;
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
    let host = secrets.get("HOST").unwrap();
    let port = secrets.get("PORT").unwrap().parse::<u16>().unwrap();
    let scheme = secrets.get("SCHEME").unwrap_or("http".into());

    let api_service = poem_openapi::OpenApiService::new(MeetingRouter {}, "API", "1.0")
        .server(format!("{scheme}://{host}:{port}/api"));
    let api_swagger = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/ui", api_swagger)
        .nest("openapi.json", spec_json)
        .with(Cors::new());

    Ok(app.into())
}
