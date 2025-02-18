use crate::domain::{meeting::Meeting, studio::StudioId};
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

pub trait MeetingRepository {
    fn create_meeting<'a>(
        &self,
        meeting: &'a Meeting,
    ) -> impl Future<Output = Result<&'a Meeting, MeetingRepositoryError>>;

    fn list_meeting(
        &self,
        studio_id: &StudioId,
    ) -> impl Future<Output = Result<Vec<Meeting>, MeetingRepositoryError>>;
}
