use poem_openapi::{payload::PlainText, OpenApi};

pub struct HelloRouter {}

#[OpenApi]
impl HelloRouter {
    #[oai(path = "/hello", method = "get")]
    pub async fn hello(&self) -> PlainText<&'static str> {
        PlainText("Hello World!")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        app::app,
        domain::meeting::Meeting,
        ports::output::meeting_repository::{MeetingRepository, MeetingRepositoryError},
    };
    use poem::test::TestClient;

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

    #[tokio::test]
    async fn test_hello() {
        let app = app("http".into(), "localhost".into(), Some(8000), MockRepo {})
            .await
            .unwrap();

        let cli = TestClient::new(app);
        let res = cli.get("/api/hello").send().await;
        res.assert_status_is_ok();
        res.assert_text("Hello World!").await;
    }
}
