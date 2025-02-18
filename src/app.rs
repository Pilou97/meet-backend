use crate::{
    adapters::input::http::handlers::{hello::HelloRouter, meeting::MeetingRouter},
    ports::output::meeting_repository::MeetingRepository,
};
use anyhow::Error;
use poem::{middleware::Cors, Endpoint, EndpointExt, Route};

pub async fn app<R>(
    scheme: String,
    host: String,
    port: Option<u16>,
    repository: R,
) -> Result<impl Endpoint, Error>
where
    R: MeetingRepository + Send + Sync + 'static,
{
    // used for swagger
    let url = match port {
        Some(port) => format!("{scheme}://{host}:{port}/api"),
        None => format!("{scheme}://{host}/api"),
    };

    let api_service = poem_openapi::OpenApiService::new(
        (MeetingRouter { repository }, HelloRouter {}),
        "API",
        "1.0",
    )
    .server(url);
    let api_swagger = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    let endpoint = Route::new()
        .nest("/api", api_service)
        .nest("/ui", api_swagger)
        .nest("openapi.json", spec_json)
        .with(Cors::new());

    Ok(endpoint)
}
