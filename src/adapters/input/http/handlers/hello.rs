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
    use std::collections::BTreeMap;

    use crate::{
        app::app,
        config::Config,
        ports::output::{meeting_repository::MockMeetingRepository, room_manager::MockRoomManager},
    };
    use poem::test::TestClient;
    use shuttle_common::secrets::Secret;
    use shuttle_runtime::SecretStore;

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
    async fn test_hello() {
        let app = app(
            config(),
            MockMeetingRepository::new(),
            MockRoomManager::new(),
        )
        .await
        .unwrap();

        let cli = TestClient::new(app);
        let res = cli.get("/api/hello").send().await;
        res.assert_status_is_ok();
        res.assert_text("Hello World!").await;
    }
}
