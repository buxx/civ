use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::space::Window;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServerEnveloppe {
    Hello(Uuid),
    Goodbye,
    Message(ClientToServerMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    SetWindow(Window),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerToClientMessage {
    Hello(u32),
}
