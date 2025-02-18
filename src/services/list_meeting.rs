use crate::{
    domain::{meeting::Meeting, studio::StudioId},
    ports::output::meeting_repository::{MeetingRepository, MeetingRepositoryError},
};
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListMeetingError {
    #[error(transparent)]
    MeetingRepository(#[from] MeetingRepositoryError),
}

pub async fn list_meeting(
    repo: &impl MeetingRepository,
    studio_id: StudioId,
    after: DateTime<Utc>,
) -> Result<Vec<Meeting>, ListMeetingError> {
    let meetings = repo
        .list_meetings(&studio_id)
        .await
        .map_err(ListMeetingError::from)?;

    let meetings = meetings
        .into_iter()
        .filter(|meeting| meeting.date > after)
        .collect();

    Ok(meetings)
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            meeting::{Meeting, MeetingId, MeetingName},
            studio::StudioId,
        },
        ports::output::meeting_repository::MockMeetingRepository,
        services::list_meeting,
    };
    use chrono::{Days, Utc};
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_filter() {
        let mut repo = MockMeetingRepository::new();
        let studio_id = StudioId::from(uuid::Uuid::new_v4());
        let old_meeting = Meeting {
            id: MeetingId::new(),
            studio_id: studio_id.clone(),
            name: MeetingName::try_from("I am an old meeting".to_string()).unwrap(),
            date: Utc::now().checked_sub_days(Days::new(2)).unwrap(),
        };
        let next_meeting = Meeting {
            id: MeetingId::new(),
            studio_id: studio_id.clone(),
            name: MeetingName::try_from("I am the fresh new one".to_string()).unwrap(),
            date: Utc::now().checked_add_days(Days::new(2)).unwrap(),
        };

        repo.expect_list_meetings()
            .with(eq(studio_id.clone()))
            .return_once(move |_| Box::pin(async { Ok(vec![old_meeting, next_meeting]) }));

        let meetings = list_meeting(&repo, studio_id, Utc::now()).await.unwrap();
        assert_eq!(meetings.len(), 1);
        assert!(meetings.iter().all(|meeting| meeting.date > Utc::now()));
        assert_eq!(
            meetings.iter().next().unwrap().name.as_ref(),
            "I am the fresh new one"
        );
    }
}
