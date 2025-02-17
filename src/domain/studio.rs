use uuid::Uuid;

pub struct StudioId(uuid::Uuid);

impl From<Uuid> for StudioId {
    fn from(uuid: Uuid) -> Self {
        StudioId(uuid)
    }
}

pub struct Studio {
    id: StudioId,
}
