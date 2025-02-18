use derive_more::AsRef;

#[derive(Debug, AsRef)]
pub struct RoomToken(String);

impl From<String> for RoomToken {
    fn from(value: String) -> Self {
        RoomToken(value)
    }
}
