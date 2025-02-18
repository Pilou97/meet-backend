use crate::domain::studio::StudioId;
use poem::{http::StatusCode, Request, RequestBody};
use poem_openapi::registry::{MetaParamIn, MetaSchemaRef};
use poem_openapi::types::Type;
use poem_openapi::{ApiExtractor, ApiExtractorType, ExtractParamOptions};
use std::str::FromStr;
use uuid::Uuid;

// impl<'a> FromRequest<'a> for StudioId {
//     async fn from_request(
//         req: &'a poem::Request,
//         _body: &mut poem::RequestBody,
//     ) -> poem::Result<Self> {
//         // TODO: we have to put some extra logic there:
//         // TODO: use JWT
//         // TODO: verify the JWT
//         // TODO: if correct, get the studio (maybe only the id of the Studio is enough)

//     }
// }

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

// #[cfg(test)]
// mod tests {
//     use crate::domain::studio::StudioId;
//     use poem::{handler, http::StatusCode, test::TestClient, Route};

//     #[handler]
//     fn authorization(_: StudioId) {}

//     #[tokio::test]
//     async fn test_auth_ok() {
//         let app = Route::new().at("/check-auth", authorization);

//         let cli = TestClient::new(app);
//         let studio_id = uuid::Uuid::new_v4();

//         let res = cli
//             .get("/check-auth")
//             .header("studio", studio_id.to_string())
//             .send()
//             .await;
//         res.assert_status_is_ok();
//     }

//     #[tokio::test]
//     async fn test_auth_ko() {
//         let app = Route::new().at("/check-auth", authorization);
//         let cli = TestClient::new(app);
//         let res = cli.get("/check-auth").send().await;
//         res.assert_status(StatusCode::UNAUTHORIZED);
//     }
// }
