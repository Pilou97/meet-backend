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
    use chrono::Utc;
    use mockall::predicate::eq;

    use crate::{
        domain::meeting::MeetingId,
        ports::output::{meeting_repository::MockMeetingRepository, room_manager::MockRoomManager},
        services::join_meeting,
    };

    use super::JoinMeetingError;

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
}
