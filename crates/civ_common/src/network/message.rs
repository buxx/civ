use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::{
        city::{CityExploitation, CityProduction},
        slice::{ClientCity, ClientUnit, GameSlice},
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
    InGame(ClientToServerInGameMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerInGameMessage {
    SetWindow(SetWindow),
    Unit(Uuid, ClientToServerUnitMessage),
    City(Uuid, ClientToServerCityMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerUnitMessage {
    Settle(String), // CityName
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerCityMessage {
    SetProduction(CityProduction),
    SetExploitation(CityExploitation),
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
    RemoveUnit(Uuid),
}
