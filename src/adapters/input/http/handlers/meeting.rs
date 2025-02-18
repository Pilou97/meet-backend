use super::super::tags::ApiTags;
use crate::{
    adapters::input::http::models::meeting::{CreateMeetingRequest, CreateMeetingResponse},
    domain::studio::StudioId,
    ports::output::meeting_repository::MeetingRepository,
    services::create_meeting,
};
use chrono::Utc;
use poem::Result;
use poem_openapi::{payload::Json, OpenApi};

pub struct MeetingRouter<R> {
    pub repository: R,
}

#[OpenApi]
impl<R> MeetingRouter<R>
where
    R: MeetingRepository + Send + Sync + 'static,
{
    #[oai(path = "/meetings", method = "post", tag = "ApiTags::Meeting")]
    pub async fn create_meeting(
        &self,
        studio_id: StudioId,
        Json(body): Json<CreateMeetingRequest>,
    ) -> Result<Json<CreateMeetingResponse>> {
        let today = Utc::now();
        let created_meeting =
            create_meeting(&self.repository, body.name, body.date, studio_id, today).await?;
        Ok(Json(created_meeting.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{app, config::Config, ports::output::meeting_repository::MockMeetingRepository};
    use poem::{http::StatusCode, test::TestClient};
    use serde::Serialize;
    use shuttle_common::secrets::Secret;
    use shuttle_runtime::SecretStore;

    #[derive(Serialize)]
    struct Body {
        name: &'static str,
        date: &'static str,
    }

    fn token() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    fn config() -> Config {
        let mut map = BTreeMap::new();
        map.insert(
            "SWAGGER_URI".to_string(),
            Secret::from("http://localhost:8000".to_string()),
        );
        map.insert(
            "DATABASE_URL".to_string(),
            Secret::from("postgres://test@test.com".to_string()),
        );
        let secrets = SecretStore::new(map);
        Config::new(secrets).unwrap()
    }

    #[tokio::test]
    pub async fn test_payload_parsing_ok() {
        let app = app(config(), MockMeetingRepository::new()).await.unwrap();

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
        res.assert_text("\"Hello meeting!!\"").await;
    }

    #[tokio::test]
    pub async fn test_payload_parsing_fail_name_is_empty() {
        let app = app(config(), MockMeetingRepository::new()).await.unwrap();

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
        let app = app(config(), MockMeetingRepository::new()).await.unwrap();

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
