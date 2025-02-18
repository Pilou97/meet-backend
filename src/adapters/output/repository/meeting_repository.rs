use super::db::Repository;
use crate::{
    domain::{
        meeting::{Meeting, MeetingId, MeetingName},
        studio::StudioId,
    },
    ports::output::meeting_repository::{MeetingRepository, MeetingRepositoryError},
};
use sqlx::query;
use validator::ValidationErrors;

impl MeetingRepository for Repository {
    async fn create_meeting(&self, meeting: &Meeting) -> Result<(), MeetingRepositoryError> {
        let meeting_id = meeting.id.as_ref();
        let studio_id = meeting.studio_id.as_ref();
        let meeting_name = meeting.name.as_ref();
        let meeting_date = meeting.date;

        query!(
            "INSERT INTO meetings (id, studio_id, name, date) VALUES ($1, $2, $3, $4)",
            meeting_id,
            studio_id,
            meeting_name,
            meeting_date
        )
        .execute(&self.pg_pool)
        .await
        .map_err(MeetingRepositoryError::from)?;

        Ok(())
    }

    async fn list_meetings(
        &self,
        studio_id: &StudioId,
    ) -> Result<Vec<Meeting>, MeetingRepositoryError> {
        let studio_id = studio_id.as_ref();
        let rows = query!(
            "SELECT id, studio_id, name, date FROM meetings where studio_id = $1",
            studio_id
        )
        .fetch_all(&self.pg_pool)
        .await
        .map_err(MeetingRepositoryError::from)?;

        rows.into_iter()
            .map(|record| {
                let name = MeetingName::try_from(record.name)?;
                Ok(Meeting {
                    id: MeetingId::from(record.id),
                    studio_id: StudioId::from(record.id),
                    name,
                    date: record.date,
                })
            })
            .collect::<Result<Vec<Meeting>, ValidationErrors>>()
            .map_err(MeetingRepositoryError::from)
    }

    async fn find_meeting(
        &self,
        meeting_id: &MeetingId,
    ) -> Result<Option<Meeting>, MeetingRepositoryError> {
        let meeting_id = meeting_id.as_ref();
        let rows = query!(
            "SELECT id, studio_id, name, date FROM meetings where id = $1",
            meeting_id
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(MeetingRepositoryError::from)?;

        let meeting = rows.map(|record| {
            let name = MeetingName::try_from(record.name)?;
            Ok(Meeting {
                id: MeetingId::from(record.id),
                studio_id: StudioId::from(record.id),
                name,
                date: record.date,
            })
        });
        match meeting {
            Some(res) => match res {
                Ok(meeting) => Ok(Some(meeting)),
                Err(err) => Err(err),
            },
            None => Ok(None),
        }
    }
}
