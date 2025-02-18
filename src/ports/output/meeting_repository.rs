use crate::domain::{meeting::Meeting, studio::StudioId};
use mockall::automock;
use std::future::Future;
use thiserror::Error;
use validator::ValidationErrors;

/// TODO: find a good way to abstract error from sqlx in trait/error definition

#[derive(Error, Debug)]
pub enum MeetingRepositoryError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

#[automock]
pub trait MeetingRepository {
    fn create_meeting(
        &self,
        meeting: &Meeting,
    ) -> impl Future<Output = Result<(), MeetingRepositoryError>> + Send;

    fn list_meetings(
        &self,
        studio_id: &StudioId,
    ) -> impl Future<Output = Result<Vec<Meeting>, MeetingRepositoryError>> + Send;
}
