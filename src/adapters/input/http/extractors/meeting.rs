use std::str::FromStr;

use crate::domain::meeting::MeetingId;
use anyhow::Context;
use poem_openapi::registry::MetaSchemaRef;
use poem_openapi::types::{ParseFromParameter, ParseResult, Type};
use uuid::Uuid;

impl Type for MeetingId {
    const IS_REQUIRED: bool = true;

    type RawValueType = uuid::Uuid;

    type RawElementValueType = uuid::Uuid;

    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("MeetingId")
    }

    fn schema_ref() -> MetaSchemaRef {
        uuid::Uuid::schema_ref()
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(&self.0)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Uuid::raw_element_iter(&self.0)
    }
}

impl ParseFromParameter for MeetingId {
    fn parse_from_parameter(value: &str) -> ParseResult<Self> {
        let uuid = uuid::Uuid::from_str(value).context("Cannot parse uuid")?;
        Ok(MeetingId::from(uuid))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::meeting::MeetingId;
    use poem::test::TestClient;
    use poem_openapi::{param::Path, OpenApi};

    struct TestRouter {}

    #[OpenApi]
    impl TestRouter {
        #[allow(unused)]
        #[oai(path = "/test/:meeting_id", method = "get")]
        async fn test(&self, #[allow(unused)] Path(meeting_id): Path<MeetingId>) {}
    }

    #[tokio::test]
    async fn test_parsing_meeting_id() {
        let api_service = poem_openapi::OpenApiService::new(TestRouter {}, "API", "1.0");
        let cli = TestClient::new(api_service);
        let meeting_id = MeetingId::new();

        let res = cli
            .get(format!("/test/{}", meeting_id.as_ref().to_string()))
            .send()
            .await;
        res.assert_status_is_ok();
    }
}
