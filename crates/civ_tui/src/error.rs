use thiserror::Error;

#[derive(Error, Debug)]
pub enum PublicError {
    #[error("Not connected to server")]
    NotConnected,
}