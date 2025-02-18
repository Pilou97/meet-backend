use derive_more::AsRef;
use uuid::Uuid;

#[derive(AsRef, Debug, PartialEq, Clone)]
pub struct StudioId(uuid::Uuid);

impl From<Uuid> for StudioId {
    fn from(uuid: Uuid) -> Self {
        StudioId(uuid)
    }
}
