use super::super::tags::ApiTags;
use poem::Result;
use poem_openapi::{payload::Json, OpenApi};

pub struct MeetingRouter {}

#[OpenApi]
impl MeetingRouter {
    #[oai(path = "/meetings", method = "post", tag = "ApiTags::Meeting")]
    pub async fn create_meeting(&self) -> Result<Json<String>> {
        Ok(Json("Hello meeting!!".to_string()))
    }
}
