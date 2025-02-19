use super::super::tags::ApiTags;
use crate::{
    adapters::input::http::models::meeting::{
        CreateMeetingRequest, CreateMeetingResponse, JoinMeetingResponse, ListMeetingsResponse,
    },
    domain::{meeting::MeetingId, studio::StudioId},
    ports::output::{meeting_repository::MeetingRepository, room_manager::RoomManager},
    services::{create_meeting, join_meeting, list_meeting},
};
use chrono::Utc;
use poem::Result;
use poem_openapi::{param::Path, payload::Json, OpenApi};

pub struct MeetingRouter<R, M> {
    pub repository: R,
    pub room_manager: M,
}

#[OpenApi]
impl<R, M> MeetingRouter<R, M>
where
    R: MeetingRepository + Send + Sync + 'static,
    M: RoomManager + Send + Sync + 'static,
{
    #[oai(path = "/meetings", method = "post", tag = "ApiTags::Meeting")]
    pub async fn create_meeting(
        &self,
        #[oai(name = "studio")] studio_id: StudioId,
        Json(body): Json<CreateMeetingRequest>,
    ) -> Result<Json<CreateMeetingResponse>> {
        let today = Utc::now();
        let created_meeting =
            create_meeting(&self.repository, body.name, body.date, studio_id, today).await?;
        Ok(Json(created_meeting.into()))
    }

    #[oai(path = "/meetings", method = "get", tag = "ApiTags::Meeting")]
    pub async fn list_meetings(
        &self,
        #[oai(name = "studio")] studio_id: StudioId,
    ) -> Result<Json<ListMeetingsResponse>> {
        let today = Utc::now();
        let meetings = list_meeting(&self.repository, studio_id, today).await?;
        Ok(Json(ListMeetingsResponse::from(meetings)))
    }

    #[oai(
        path = "/meetings/:meeting-id/join",
        method = "get",
        tag = "ApiTags::Meeting"
    )]
    pub async fn join_meeting(
        &self,
        Path(meeting_id): Path<MeetingId>,
    ) -> Result<Json<JoinMeetingResponse>> {
        let today = Utc::now();
        let token = join_meeting(&self.repository, &self.room_manager, meeting_id, today).await?;
        Ok(Json(JoinMeetingResponse::from(token)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::studio::StudioId,
        ports::output::{
            config::MockConfig, meeting_repository::MockMeetingRepository,
            room_manager::MockRoomManager,
        },
    };
    use chrono::{Days, Utc};
    use mockall::predicate::eq;
    use poem::{http::StatusCode, test::TestClient};
    use serde::Serialize;

    #[derive(Serialize)]
    struct Body<'a> {
        name: &'a str,
        date: &'a str,
    }

    fn token(studio_id: StudioId) -> String {
        studio_id.as_ref().to_string()
    }

    #[tokio::test]
    pub async fn test_payload_parsing_ok() {
        let studio_id = StudioId::from(uuid::Uuid::new_v4());

        let mut mock_repo = MockMeetingRepository::new();
        mock_repo
            .expect_create_meeting()
            .once()
            .return_once(|_| Box::pin(async { Ok(()) }));

        mock_repo
            .expect_list_meetings()
            .once()
            .with(eq(studio_id.clone()))
            .return_once(|_| Box::pin(async { Ok(vec![]) }));

        let app = crate::app::app(MockConfig::new(), mock_repo, MockRoomManager::new())
            .await
            .unwrap();

        let cli = TestClient::new(app);

        let date = Utc::now()
            .checked_add_days(Days::new(2))
            .unwrap()
            .to_string();

        let res = cli
            .post("/api/meetings")
            .body_json(&Body {
                name: "Meeting name",
                date: date.as_str(),
            })
            .header("studio", token(studio_id))
            .send()
            .await;
        res.assert_status_is_ok();
    }

    #[tokio::test]
    pub async fn test_payload_parsing_fail_name_is_empty() {
        let app = crate::app::app(
            MockConfig::new(),
            MockMeetingRepository::new(),
            MockRoomManager::new(),
        )
        .await
        .unwrap();

        let cli = TestClient::new(app);
        let res = cli
            .post("/api/meetings")
            .body_json(&Body {
                name: "",
                date: "2025-02-17T17:50:41.777Z",
            })
            .header("studio", token(StudioId::from(uuid::Uuid::new_v4())))
            .send()
            .await;
        res.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    pub async fn test_authorization_is_needed() {
        let app = crate::app::app(
            MockConfig::new(),
            MockMeetingRepository::new(),
            MockRoomManager::new(),
        )
        .await
        .unwrap();

        let cli = TestClient::new(app);
        let res = cli
            .post("/api/meetings")
            .body_json(&Body {
                name: "Hello meeting",
                date: "2025-02-17T17:50:41.777Z",
            })
            .send()
            .await;
        res.assert_status(StatusCode::UNAUTHORIZED);
    }
}
