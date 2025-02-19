use crate::services::{CreateMeetingError, JoinMeetingError, ListMeetingError};
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

impl ResponseError for JoinMeetingError {
    fn status(&self) -> StatusCode {
        match self {
            JoinMeetingError::NotFound => StatusCode::NOT_FOUND,
            JoinMeetingError::TooLate => StatusCode::BAD_REQUEST,
            JoinMeetingError::MeetingRepository(_) => StatusCode::INTERNAL_SERVER_ERROR,
            JoinMeetingError::RoomManager(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
