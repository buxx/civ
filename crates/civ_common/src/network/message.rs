use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::slice::GameSlice,
    space::window::{SetWindow, Window},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerEnveloppe {
    Hello(Uuid),
    Goodbye,
    Message(ClientToServerMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    SetWindow(SetWindow),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerToClientMessage {
    State(ClientStateMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientStateMessage {
    SetWindow(Window),
    SetGameSlice(GameSlice),
}
