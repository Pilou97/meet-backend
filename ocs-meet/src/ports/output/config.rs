use mockall::automock;

#[automock]
pub trait Config {
    fn swagger_uri(&self) -> url::Url;
    fn livekit_api_key(&self) -> String;
    fn livekit_secret(&self) -> String;
}
