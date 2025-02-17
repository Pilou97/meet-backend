use crate::domain::studio::StudioId;

use super::super::tags::ApiTags;
use poem::Result;
use poem_openapi::{payload::Json, OpenApi};

pub struct MeetingRouter {}

#[OpenApi]
impl MeetingRouter {
    #[oai(path = "/meetings", method = "post", tag = "ApiTags::Meeting")]
    pub async fn create_meeting(&self, _studio: StudioId) -> Result<Json<String>> {
        Ok(Json("Hello meeting!!".to_string()))
    }
}
