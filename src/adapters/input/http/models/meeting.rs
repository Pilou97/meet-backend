use crate::domain::meeting::{Meeting, MeetingName};
use chrono::{DateTime, Utc};
use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{ParseError, ParseFromJSON, ParseResult, ToJSON, Type},
    Object,
};
use serde_json::Value;

#[derive(Object)]
pub struct CreateMeetingRequest {
    pub name: MeetingName,
    pub date: DateTime<Utc>,
}

impl Type for MeetingName {
    const IS_REQUIRED: bool = true;

    type RawValueType = String;

    type RawElementValueType = String;

    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("MeetingName")
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "uuid")))
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self.as_ref())
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl ParseFromJSON for MeetingName {
    fn parse_from_json(value: Option<Value>) -> poem_openapi::types::ParseResult<Self> {
        let value = value.unwrap_or_default();
        if let Value::String(string) = value {
            let meeting_name =
                MeetingName::try_from(string).map_err(|err| ParseError::from(err))?;
            ParseResult::Ok(meeting_name)
        } else {
            return ParseResult::Err(ParseError::expected_type(value));
        }
    }
}

impl ToJSON for MeetingName {
    fn to_json(&self) -> Option<Value> {
        Some(Value::String(self.as_ref().to_string()))
    }
}

#[derive(Object)]
pub struct MeetingHttp {
    id: uuid::Uuid,
    studio_id: uuid::Uuid,
    name: MeetingName,
    date: DateTime<Utc>,
}

pub type CreateMeetingResponse = MeetingHttp;

impl From<Meeting> for CreateMeetingResponse {
    fn from(value: Meeting) -> Self {
        CreateMeetingResponse {
            id: value.id.as_ref().clone(),
            studio_id: value.studio_id.as_ref().clone(),
            name: value.name,
            date: value.date,
        }
    }
}

#[derive(Object)]
pub struct ListMeetingsResponse {
    meetings: Vec<MeetingHttp>,
}

impl From<Vec<Meeting>> for ListMeetingsResponse {
    fn from(meetings: Vec<Meeting>) -> Self {
        ListMeetingsResponse {
            meetings: meetings.into_iter().map(MeetingHttp::from).collect(),
        }
    }
}
