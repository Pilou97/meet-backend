use chrono::{DateTime, Utc};
use derive_more::AsRef;
use validator::{Validate, ValidationErrors};

use super::studio::StudioId;

#[derive(AsRef)]
pub struct MeetingId(uuid::Uuid);

impl From<uuid::Uuid> for MeetingId {
    fn from(id: uuid::Uuid) -> Self {
        MeetingId(id)
    }
}

#[derive(Validate, AsRef)]
pub struct MeetingName {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    inner: String,
}

impl TryFrom<String> for MeetingName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let name = MeetingName { inner: value };
        let () = name.validate()?;
        Ok(name)
    }
}

pub struct Meeting {
    pub id: MeetingId,
    pub studio_id: StudioId,
    pub name: MeetingName,
    pub date: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::MeetingName;

    #[test]
    fn test_name_length_ko() {
        let raw_name = "".to_string();
        let name = MeetingName::try_from(raw_name);
        assert!(name.is_err());
    }

    #[test]
    fn test_name_length_ok() {
        let raw_name = "Hello meeting".to_string();
        let name = MeetingName::try_from(raw_name);
        assert!(name.is_ok());
    }
}
