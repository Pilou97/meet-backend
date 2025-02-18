use super::repository::Repository;
use crate::{
    domain::{meeting::Meeting, studio::StudioId},
    ports::output::meeting_repository::{MeetingRepository, MeetingRepositoryError},
};

impl MeetingRepository for Repository {
    async fn create_meeting<'a>(
        &self,
        _meeting: &'a Meeting,
    ) -> Result<&'a Meeting, MeetingRepositoryError> {
        todo!()
    }

    async fn list_meeting(
        &self,
        _studio_id: &StudioId,
    ) -> Result<Vec<Meeting>, MeetingRepositoryError> {
        todo!()
    }
}
