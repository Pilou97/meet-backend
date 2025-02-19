use crate::{
    domain::{meeting::MeetingId, room::RoomToken},
    ports::output::{
        meeting_repository::{MeetingRepository, MeetingRepositoryError},
        room_manager::{RoomManager, RoomManagerError},
    },
};
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JoinMeetingError {
    #[error("The meeting you're looking for does not exist")]
    NotFound,
    #[error("It's too late to join the meeting")]
    TooLate,
    #[error(transparent)]
    MeetingRepository(#[from] MeetingRepositoryError),
    #[error(transparent)]
    RoomManager(#[from] RoomManagerError),
}

pub async fn join_meeting(
    repository: &impl MeetingRepository,
    room_manager: &impl RoomManager,
    meeting_id: MeetingId,
    today: DateTime<Utc>,
) -> Result<RoomToken, JoinMeetingError> {
    let Some(meeting) = repository
        .find_meeting(&meeting_id)
        .await
        .map_err(JoinMeetingError::from)?
    else {
        return Err(JoinMeetingError::NotFound);
    };

    if meeting.date < today {
        return Err(JoinMeetingError::TooLate);
    }

    room_manager
        .create_token(meeting.id)
        .await
        .map_err(JoinMeetingError::from)
}

#[cfg(test)]
mod tests {
    use super::JoinMeetingError;
    use crate::{
        domain::{
            meeting::{Meeting, MeetingId, MeetingName},
            room::RoomToken,
            studio::StudioId,
        },
        ports::output::{meeting_repository::MockMeetingRepository, room_manager::MockRoomManager},
        services::join_meeting,
    };
    use chrono::{Days, Utc};
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_not_found() {
        let meeting_id = MeetingId::new();

        let mut mock_repo = MockMeetingRepository::new();
        let mock_room_manager = MockRoomManager::new();

        mock_repo
            .expect_find_meeting()
            .once()
            .with(eq(meeting_id.clone()))
            .return_once(|_| Box::pin(async { Ok(None) }));

        let Err(JoinMeetingError::NotFound) =
            join_meeting(&mock_repo, &mock_room_manager, meeting_id, Utc::now()).await
        else {
            panic!("The meeting should not be found");
        };
    }

    #[tokio::test]
    async fn test_too_late() {
        let meeting_id = MeetingId::new();

        let mut mock_repo = MockMeetingRepository::new();
        let mock_room_manager = MockRoomManager::new();

        mock_repo
            .expect_find_meeting()
            .once()
            .with(eq(meeting_id.clone()))
            .return_once(move |meeting_id| {
                let meeting_id = meeting_id.clone();
                Box::pin(async move {
                    Ok(Some(Meeting {
                        id: meeting_id,
                        studio_id: StudioId::from(uuid::Uuid::new_v4()),
                        name: MeetingName::try_from("An old meeting".to_string()).unwrap(),
                        date: Utc::now().checked_sub_days(Days::new(2)).unwrap(),
                    }))
                })
            });

        let Err(JoinMeetingError::TooLate) =
            join_meeting(&mock_repo, &mock_room_manager, meeting_id, Utc::now()).await
        else {
            panic!("The meeting should exist but in the past");
        };
    }

    #[tokio::test]
    async fn test_ok() {
        let meeting_id = MeetingId::new();

        let mut mock_repo = MockMeetingRepository::new();
        let mut mock_room_manager = MockRoomManager::new();

        mock_repo
            .expect_find_meeting()
            .once()
            .with(eq(meeting_id.clone()))
            .return_once(move |meeting_id| {
                let meeting_id = meeting_id.clone();
                Box::pin(async move {
                    Ok(Some(Meeting {
                        id: meeting_id,
                        studio_id: StudioId::from(uuid::Uuid::new_v4()),
                        name: MeetingName::try_from("Hello there!".to_string()).unwrap(),
                        date: Utc::now().checked_add_days(Days::new(2)).unwrap(),
                    }))
                })
            });

        mock_room_manager
            .expect_create_token()
            .once()
            .with(eq(meeting_id.clone()))
            .return_once(|_| Box::pin(async { Ok(RoomToken::from("mytoken".to_string())) }));

        let Ok(_) = join_meeting(&mock_repo, &mock_room_manager, meeting_id, Utc::now()).await
        else {
            panic!("A token should be returned");
        };
    }
}
