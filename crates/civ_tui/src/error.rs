use common::network::message::TakePlaceRefusedReason;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PublicError {
    #[error("Not connected to server")]
    NotConnected,
    #[error("Cant take place because: {0}")]
    CantTakePlace(TakePlaceRefusedReason),
    #[error("{0}")]
    ServerNotification(String),
}
