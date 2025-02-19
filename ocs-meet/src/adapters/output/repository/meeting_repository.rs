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

#[cfg(test)]
mod tests {
    use crate::{
        adapters::output::repository::db::Repository,
        domain::{
            meeting::{Meeting, MeetingId, MeetingName},
            studio::StudioId,
        },
        ports::output::meeting_repository::MeetingRepository,
    };
    use chrono::Utc;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_meeting(pg_pool: PgPool) {
        let repository = Repository::new(pg_pool).await.unwrap();
        let meeting = Meeting {
            id: MeetingId::new(),
            studio_id: StudioId::from(uuid::Uuid::new_v4()),
            name: MeetingName::try_from("Hello meeting".to_string()).unwrap(),
            date: Utc::now(),
        };
        repository.create_meeting(&meeting).await.unwrap();
        let Some(retrieved) = repository.find_meeting(&meeting.id).await.unwrap() else {
            panic!("A meeting should have been created")
        };
        assert_eq!(retrieved.id, meeting.id);
    }

    #[sqlx::test]
    async fn test_list_meetings(pg_pool: PgPool) {
        let repository = Repository::new(pg_pool).await.unwrap();
        let studio_id = StudioId::from(uuid::Uuid::new_v4());
        let other_studio = StudioId::from(uuid::Uuid::new_v4());

        let meeting_one = Meeting {
            id: MeetingId::new(),
            studio_id: studio_id.clone(),
            name: MeetingName::try_from("Hello meeting".to_string()).unwrap(),
            date: Utc::now(),
        };
        let meeting_two = Meeting {
            id: MeetingId::new(),
            studio_id: studio_id.clone(),
            name: MeetingName::try_from("Hello meeting".to_string()).unwrap(),
            date: Utc::now(),
        };
        repository.create_meeting(&meeting_one).await.unwrap();
        repository.create_meeting(&meeting_two).await.unwrap();

        let list = repository.list_meetings(&studio_id).await.unwrap();
        let list_2 = repository.list_meetings(&other_studio).await.unwrap();

        assert!(!list.is_empty());
        assert!(list_2.is_empty());
    }
}
