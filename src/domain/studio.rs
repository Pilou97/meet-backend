use derive_more::AsRef;
use uuid::Uuid;

#[derive(AsRef)]
pub struct StudioId(uuid::Uuid);

impl From<Uuid> for StudioId {
    fn from(uuid: Uuid) -> Self {
        StudioId(uuid)
    }
}

pub struct Studio {
    id: StudioId,
}
