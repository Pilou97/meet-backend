use std::future::Future;

use livekit_api::access_token::AccessTokenError;
use mockall::automock;
use thiserror::Error;

use crate::domain::{meeting::MeetingId, room::RoomToken};

#[derive(Debug, Error)]
pub enum RoomManagerError {
    #[error(transparent)]
    LiveKitAccessToken(#[from] AccessTokenError),
}

#[automock]
pub trait RoomManager {
    fn create_token(
        &self,
        meeting_id: MeetingId,
    ) -> impl Future<Output = Result<RoomToken, RoomManagerError>> + Send;
}
