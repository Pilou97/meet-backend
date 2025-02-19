use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::{
    domain::{
        meeting::{Meeting, MeetingId, MeetingName},
        studio::StudioId,
    },
    ports::output::meeting_repository::{MeetingRepository, MeetingRepositoryError},
};

#[derive(Debug, Error)]
pub enum CreateMeetingError {
    #[error("Cannot create a meeting in the past")]
    DateInThePast,
    #[error("A meeting with the same date already exists")]
    DateAlreadyTaken,
    #[error(transparent)]
    MeetingRepository(#[from] MeetingRepositoryError),
}

pub async fn create_meeting(
    repo: &impl MeetingRepository,
    name: MeetingName,
    date: DateTime<Utc>,
    studio_id: StudioId,
    today: DateTime<Utc>,
) -> Result<Meeting, CreateMeetingError> {
    if date < today {
        return Err(CreateMeetingError::DateInThePast);
    }

    let meetings = repo
        .list_meetings(&studio_id)
        .await
        .map_err(CreateMeetingError::from)?;

    let None = meetings.iter().find(|meeting| meeting.date == date) else {
        return Err(CreateMeetingError::DateAlreadyTaken);
    };

    let id = MeetingId::new();
    let meeting = Meeting {
        id,
        name,
        date,
        studio_id,
    };

    repo.create_meeting(&meeting)
        .await
        .map_err(CreateMeetingError::from)?;

    Ok(meeting)
}

#[cfg(test)]
mod test {
    use chrono::{Days, Timelike, Utc};
    use mockall::predicate::eq;

    use crate::{
        domain::{
            meeting::{Meeting, MeetingId, MeetingName},
            studio::StudioId,
        },
        ports::output::meeting_repository::MockMeetingRepository,
        services::{create_meeting::create_meeting, CreateMeetingError},
    };

    #[tokio::test]
    async fn test_create_meeting_ok() {
        let name = MeetingName::try_from("Hello meeting".to_string()).unwrap();
        let date: chrono::DateTime<Utc> = Utc::now();
        let studio_id = StudioId::from(uuid::Uuid::new_v4());

        let today = date.with_hour(0).unwrap();

        let mut mock_repo = MockMeetingRepository::new();
        mock_repo
            .expect_create_meeting()
            .once()
            .returning(|_| Box::pin(async { Ok(()) }));
        mock_repo
            .expect_list_meetings()
            .once()
            .with(eq(studio_id.clone()))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        let _meeting = create_meeting(&mock_repo, name, date, studio_id, today)
            .await
            .expect("The meeting should be created");
    }

    #[tokio::test]
    async fn test_create_meeting_date_in_the_past() {
        let name = MeetingName::try_from("Hello meeting".to_string()).unwrap();
        let date: chrono::DateTime<Utc> = Utc::now().checked_sub_days(Days::new(2)).unwrap();
        let studio_id = StudioId::from(uuid::Uuid::new_v4());
        let today = Utc::now();

        let mut mock_repo = MockMeetingRepository::new();
        mock_repo
            .expect_create_meeting()
            .never()
            .returning(|_| Box::pin(async { Ok(()) }));
        mock_repo
            .expect_list_meetings()
            .never()
            .with(eq(studio_id.clone()))
            .returning(|_| Box::pin(async { Ok(vec![]) }));

        let Err(CreateMeetingError::DateInThePast) =
            create_meeting(&mock_repo, name, date, studio_id, today).await
        else {
            panic!("The meeting should not be created")
        };
    }

    #[tokio::test]
    async fn test_create_meeting_same_date() {
        let name = MeetingName::try_from("Hello meeting".to_string()).unwrap();
        let date: chrono::DateTime<Utc> = Utc::now();
        let studio_id = StudioId::from(uuid::Uuid::new_v4());
        let today = date.with_hour(0).unwrap();

        let mut mock_repo = MockMeetingRepository::new();
        mock_repo
            .expect_create_meeting()
            .never()
            .returning(|_| Box::pin(async { Ok(()) }));
        mock_repo
            .expect_list_meetings()
            .once()
            .with(eq(studio_id.clone()))
            .returning(move |studio_id| {
                let studio_id = studio_id.clone();
                let date = date;
                Box::pin(async move {
                    Ok(vec![Meeting {
                        id: MeetingId::new(),
                        studio_id,
                        name: MeetingName::try_from("Another meeting".to_string()).unwrap(),
                        date,
                    }])
                })
            });

        let Err(CreateMeetingError::DateAlreadyTaken) =
            create_meeting(&mock_repo, name, date, studio_id, today).await
        else {
            panic!("The meeting should not be created")
        };
    }
}
