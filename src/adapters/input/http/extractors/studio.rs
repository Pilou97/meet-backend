use crate::domain::studio::StudioId;
use poem::{http::StatusCode, Request, RequestBody};
use poem_openapi::registry::{MetaParamIn, MetaSchemaRef};
use poem_openapi::types::Type;
use poem_openapi::{ApiExtractor, ApiExtractorType, ExtractParamOptions};
use std::str::FromStr;
use uuid::Uuid;

impl<'a> ApiExtractor<'a> for StudioId {
    const TYPES: &'static [ApiExtractorType] = &[ApiExtractorType::Parameter];
    const PARAM_IS_REQUIRED: bool = true;

    type ParamType = String;
    type ParamRawType = Uuid;

    async fn from_request(
        request: &'a Request,
        _body: &mut RequestBody,
        _param_opts: ExtractParamOptions<Self::ParamType>,
    ) -> poem::Result<Self> {
        let Some(studio) = request.header("studio") else {
            println!("no token");
            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
        };

        let Ok(uuid) = uuid::Uuid::from_str(studio) else {
            return Err(poem::Error::from_status(StatusCode::BAD_REQUEST));
        };

        Ok(StudioId::from(uuid))
    }

    /// Returns the location of the parameter if this extractor is parameter.
    fn param_in() -> Option<MetaParamIn> {
        Some(MetaParamIn::Header)
    }

    /// Returns the schema of the parameter if this extractor is parameter.
    fn param_schema_ref() -> Option<MetaSchemaRef> {
        Some(Uuid::schema_ref())
    }

    /// Returns a reference to the raw type of this parameter.
    fn param_raw_type(&self) -> Option<&Self::ParamRawType> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::studio::StudioId;
    use poem::{http::StatusCode, test::TestClient};
    use poem_openapi::OpenApi;

    struct TestRouter {}

    #[OpenApi]
    impl TestRouter {
        #[oai(path = "/test", method = "get")]
        async fn test(&self, _studio_id: StudioId) {}
    }

    #[tokio::test]
    async fn test_auth_ok() {
        let api_service = poem_openapi::OpenApiService::new(TestRouter {}, "API", "1.0");
        let cli = TestClient::new(api_service);

        let studio_id = uuid::Uuid::new_v4();

        let res = cli
            .get("/test")
            .header("studio", studio_id.to_string())
            .send()
            .await;
        res.assert_status_is_ok();
    }

    #[tokio::test]
    async fn test_auth_ko() {
        let api_service = poem_openapi::OpenApiService::new(TestRouter {}, "API", "1.0");
        let cli = TestClient::new(api_service);

        let res = cli.get("/test").send().await;
        res.assert_status(StatusCode::UNAUTHORIZED);
    }
}
