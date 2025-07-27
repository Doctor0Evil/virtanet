#[derive(Debug)]
pub enum SystemError {
    Io(std::io::Error),
    TiKv(tikv_client::Error),
    Nats(nats::Error),
    Generic(String),
}
impl From<std::io::Error> for SystemError {
    fn from(e: std::io::Error) -> Self {
        SystemError::Io(e)
    }
}
impl From<tikv_client::Error> for SystemError {
    fn from(e: tikv_client::Error) -> Self {
        SystemError::TiKv(e)
    }
}
impl From<String> for SystemError {
    fn from(e: String) -> Self {
        SystemError::Generic(e)
    }
}
impl From<nats::Error> for SystemError {
    fn from(e: nats::Error) -> Self {
        SystemError::Nats(e)
    }
}
pub type Result<T> = std::result::Result<T, SystemError>;