use super::super::tags::ApiTags;
use crate::{
    adapters::{
        input::http::models::meeting::CreateMeetingRequest, output::repository::Repository,
    },
    domain::studio::StudioId,
};
use poem::{web::Data, Result};
use poem_openapi::{payload::Json, OpenApi};

pub struct MeetingRouter {}

#[OpenApi]
impl MeetingRouter {
    #[oai(path = "/meetings", method = "post", tag = "ApiTags::Meeting")]
    pub async fn create_meeting(
        &self,
        _studio: StudioId,
        _body: Json<CreateMeetingRequest>,
        _db: Data<&Repository>,
    ) -> Result<Json<String>> {
        Ok(Json("Hello meeting!!".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use poem::{http::StatusCode, test::TestClient};
    use serde::Serialize;

    use crate::app;

    #[derive(Serialize)]
    struct Body {
        name: &'static str,
        date: &'static str,
    }

    fn token() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    #[tokio::test]
    pub async fn test_payload_parsing_ok() {
        let app = app("http".into(), "localhost".into(), Some(8000));

        let cli = TestClient::new(app);
        let res = cli
            .post("/api/meetings")
            .body_json(&Body {
                name: "Meeting name",
                date: "2025-02-17T17:50:41.777Z",
            })
            .header("studio", token())
            .send()
            .await;
        res.assert_status_is_ok();
    }

    #[tokio::test]
    pub async fn test_payload_parsing_fail_name_is_empty() {
        let app = app("http".into(), "localhost".into(), Some(8000));

        let cli = TestClient::new(app);
        let res = cli
            .post("/api/meetings")
            .body_json(&Body {
                name: "",
                date: "2025-02-17T17:50:41.777Z",
            })
            .header("studio", token())
            .send()
            .await;
        res.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    pub async fn test_authorization() {
        let app = app("http".into(), "localhost".into(), Some(8000));

        let cli = TestClient::new(app);
        let res = cli
            .post("/api/meetings")
            .body_json(&Body {
                name: "Hello meeting",
                date: "2025-02-17T17:50:41.777Z",
            })
            .send()
            .await;
        res.assert_status(StatusCode::UNAUTHORIZED);
    }
}
