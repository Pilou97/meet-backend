use super::super::tags::ApiTags;
use crate::{
    adapters::input::http::models::meeting::CreateMeetingRequest, domain::studio::StudioId,
    ports::output::meeting_repository::MeetingRepository,
};
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
        _studio: StudioId,
        _body: Json<CreateMeetingRequest>,
    ) -> Result<Json<String>> {
        Ok(Json("Hello meeting!!".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        app,
        config::Config,
        domain::meeting::Meeting,
        ports::output::meeting_repository::{MeetingRepository, MeetingRepositoryError},
    };
    use poem::{http::StatusCode, test::TestClient};
    use serde::Serialize;
    use shuttle_common::secrets::Secret;
    use shuttle_runtime::SecretStore;

    #[derive(Clone)]
    struct MockRepo {}

    /// TODO: see if can derive it with a crate or not
    impl MeetingRepository for MockRepo {
        async fn create_meeting<'a>(
            &self,
            _meeting: &'a Meeting,
        ) -> Result<&'a Meeting, MeetingRepositoryError> {
            todo!()
        }

        async fn list_meeting(
            &self,
            _studio_id: &crate::domain::studio::StudioId,
        ) -> Result<Vec<Meeting>, MeetingRepositoryError> {
            todo!()
        }
    }

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
            "DATABASE_URI".to_string(),
            Secret::from("postgres://test@test.com".to_string()),
        );
        let secrets = SecretStore::new(map);
        Config::new(secrets).unwrap()
    }

    #[tokio::test]
    pub async fn test_payload_parsing_ok() {
        let app = app(config(), MockRepo {}).await.unwrap();

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
        let app = app(config(), MockRepo {}).await.unwrap();

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
        let app = app(config(), MockRepo {}).await.unwrap();

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
