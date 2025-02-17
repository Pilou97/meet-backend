use std::str::FromStr;

use poem::{http::StatusCode, FromRequest};

use crate::domain::studio::StudioId;

impl<'a> FromRequest<'a> for StudioId {
    async fn from_request(
        req: &'a poem::Request,
        _body: &mut poem::RequestBody,
    ) -> poem::Result<Self> {
        // TODO: we have to put some extra logic there:
        // TODO: use JWT
        // TODO: verify the JWT
        // TODO: if correct, get the studio (maybe only the id of the Studio is enough)
        let Some(studio) = req.header("studio") else {
            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
        };

        let Ok(uuid) = uuid::Uuid::from_str(studio) else {
            return Err(poem::Error::from_status(StatusCode::BAD_REQUEST));
        };

        Ok(StudioId::from(uuid))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::studio::StudioId;
    use poem::{handler, http::StatusCode, test::TestClient, Route};

    #[handler]
    fn authorization(_: StudioId) {}

    #[tokio::test]
    async fn test_auth_ok() {
        let app = Route::new().at("/check-auth", authorization);

        let cli = TestClient::new(app);
        let studio_id = uuid::Uuid::new_v4();

        let res = cli
            .get("/check-auth")
            .header("studio", studio_id.to_string())
            .send()
            .await;
        res.assert_status_is_ok();
    }

    #[tokio::test]
    async fn test_auth_ko() {
        let app = Route::new().at("/check-auth", authorization);
        let cli = TestClient::new(app);
        let res = cli.get("/check-auth").send().await;
        res.assert_status(StatusCode::UNAUTHORIZED);
    }
}
