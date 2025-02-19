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
        config::Config,
        ports::output::{meeting_repository::MockMeetingRepository, room_manager::MockRoomManager},
    };
    use poem::test::TestClient;

    #[tokio::test]
    async fn test_hello() {
        let app = app(
            Config::create_mock(),
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
