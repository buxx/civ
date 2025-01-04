use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::{
        slice::{ClientCity, ClientUnit, ClientConcreteTask, GameSlice},
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
    SetCity(ClientCity),
    RemoveCity(Uuid),
    SetUnit(ClientUnit),
    AddUnitTask(Uuid, ClientConcreteTask),
    RemoveUnitTask(Uuid, Uuid),
    RemoveUnit(Uuid),
}
