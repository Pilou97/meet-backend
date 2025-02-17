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
    use poem::test::TestClient;

    use crate::app::app;

    #[tokio::test]
    async fn test_hello() {
        let app = app("http".into(), "localhost".into(), Some(8000));

        let cli = TestClient::new(app);
        let res = cli.get("/api/hello").send().await;
        res.assert_status_is_ok();
        res.assert_text("Hello World!").await;
    }
}
