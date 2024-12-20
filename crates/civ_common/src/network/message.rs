use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::{
        slice::{ClientCity, ClientTask, ClientUnit, GameSlice},
        GameFrame,
    },
    space::window::{SetWindow, Window},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
    CreateTask(Uuid, CreateTaskMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CreateTaskMessage {
    Settle(Uuid, String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServerToClientMessage {
    State(ClientStateMessage),
    Notification(NotificationLevel, String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClientStateMessage {
    SetGameFrame(GameFrame),
    SetWindow(Window),
    SetGameSlice(GameSlice),
    AddCity(ClientCity),
    RemoveCity(Uuid),
    AddUnit(ClientUnit),
    AddUnitTask(Uuid, ClientTask),
    RemoveUnitTask(Uuid, Uuid),
    MoveUnit(Uuid, (u64, u64)), // FIXME resend whole unit when small change
    RemoveUnit(Uuid),
}
