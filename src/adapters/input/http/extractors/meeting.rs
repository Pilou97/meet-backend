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
