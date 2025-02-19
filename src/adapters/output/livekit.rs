use crate::{
    domain::{meeting::MeetingId, room::RoomToken},
    ports::output::room_manager::{RoomManager, RoomManagerError},
};
use livekit_api::access_token;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Livekit {
    api_key: Arc<String>,
    api_secret: Arc<String>,
}

impl Livekit {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key: Arc::new(api_key),
            api_secret: Arc::new(api_secret),
        }
    }
}

impl RoomManager for Livekit {
    async fn create_token(&self, meeting_id: MeetingId) -> Result<RoomToken, RoomManagerError> {
        let identity = uuid::Uuid::new_v4();
        let token = access_token::AccessToken::with_api_key(&self.api_key, &self.api_secret)
            .with_identity(&identity.to_string())
            .with_grants(access_token::VideoGrants {
                room_join: true,
                room: meeting_id.as_ref().to_string(),
                ..Default::default()
            });
        token
            .to_jwt()
            .map_err(RoomManagerError::LiveKitAccessToken)
            .map(RoomToken::from)
    }
}
