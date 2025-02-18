use crate::services::CreateMeetingError;
use poem::{error::ResponseError, http::StatusCode};

impl ResponseError for CreateMeetingError {
    fn status(&self) -> StatusCode {
        match self {
            CreateMeetingError::DateInThePast => StatusCode::BAD_REQUEST,
            CreateMeetingError::DateAlreadyTaken => StatusCode::BAD_REQUEST,
            CreateMeetingError::MeetingRepository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
