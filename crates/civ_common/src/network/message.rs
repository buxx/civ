use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
    space::window::{SetWindow, Window},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum NotificationLevel {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerEnveloppe {
    Hello(Uuid),
    Goodbye,
    Message(ClientToServerMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    SetWindow(SetWindow),
    CreateTask(CreateTaskMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CreateTaskMessage {
    Settle(Uuid, String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerToClientMessage {
    State(ClientStateMessage),
    Notification(NotificationLevel, String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientStateMessage {
    SetWindow(Window),
    SetGameSlice(GameSlice),
    AddCity(ClientCity),
    RemoveCity(Uuid),
    AddUnit(ClientUnit),
    MoveUnit(Uuid, (u64, u64)),
    RemoveUnit(Uuid),
}
