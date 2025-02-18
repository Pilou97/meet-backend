use crate::services::{CreateMeetingError, ListMeetingError};
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

impl ResponseError for ListMeetingError {
    fn status(&self) -> StatusCode {
        match self {
            ListMeetingError::MeetingRepository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
