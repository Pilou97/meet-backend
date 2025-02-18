use crate::domain::{meeting::Meeting, studio::StudioId};
use std::future::Future;
use thiserror::Error;

/// TODO: find a good way to abstract error from sqlx in trait/error definition

#[derive(Error, Debug)]
pub enum MeetingRepositoryError {}

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
