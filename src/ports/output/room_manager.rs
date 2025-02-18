use std::future::Future;

use mockall::automock;
use thiserror::Error;

use crate::domain::{meeting::MeetingId, room::RoomToken};

#[derive(Debug, Error)]
pub enum RoomManagerError {}

#[automock]
pub trait RoomManager {
    fn create_token(
        &self,
        meeting_id: MeetingId,
    ) -> impl Future<Output = Result<RoomToken, RoomManagerError>>;
}
