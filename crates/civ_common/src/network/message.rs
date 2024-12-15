use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::slice::GameSlice,
    space::window::{SetWindow, Window},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServerEnveloppe {
    Hello(Uuid),
    Goodbye,
    Message(ClientToServerMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    SetWindow(SetWindow),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerToClientMessage {
    State(ClientStateMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientStateMessage {
    SetWindow(Window),
    SetGameSlice(GameSlice),
}
