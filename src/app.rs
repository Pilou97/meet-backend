use crate::adapters::input::http::handlers::{hello::HelloRouter, meeting::MeetingRouter};
use poem::{middleware::Cors, Endpoint, EndpointExt, Route};

pub fn app(scheme: String, host: String, port: Option<u16>) -> impl Endpoint {
    // used for swagger
    let url = match port {
        Some(port) => format!("{scheme}://{host}:{port}/api"),
        None => format!("{scheme}://{host}/api"),
    };

    let api_service =
        poem_openapi::OpenApiService::new((MeetingRouter {}, HelloRouter {}), "API", "1.0")
            .server(url);
    let api_swagger = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    Route::new()
        .nest("/api", api_service)
        .nest("/ui", api_swagger)
        .nest("openapi.json", spec_json)
        .with(Cors::new())
}
