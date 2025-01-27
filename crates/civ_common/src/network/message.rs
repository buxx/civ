use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::{
        city::{CityExploitation, CityId, CityProduction},
        nation::flag::Flag,
        server::ServerResume,
        slice::{ClientCity, ClientUnit, GameSlice},
        unit::UnitId,
        GameFrame,
    },
    space::window::{SetWindow, Window, Resolution},
};

use super::Client;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NotificationLevel {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    Network(ClientToServerNetworkMessage),
    Game(ClientToServerGameMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerNetworkMessage {
    Hello(Client, Resolution),
    Goodbye,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerGameMessage {
    Establishment(ClientToServerEstablishmentMessage),
    InGame(ClientToServerInGameMessage),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerEstablishmentMessage {
    TakePlace(Flag),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientToServerInGameMessage {
    SetWindow(SetWindow),
    Unit(UnitId, ClientToServerUnitMessage),
    City(CityId, ClientToServerCityMessage),
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
    Establishment(ServerToClientEstablishmentMessage),
    InGame(ServerToClientInGameMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServerToClientEstablishmentMessage {
    ServerResume(ServerResume, Option<Flag>), // None flag mean player not placed
    TakePlaceRefused(TakePlaceRefusedReason),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServerToClientInGameMessage {
    State(ClientStateMessage),
    Notification(NotificationLevel, String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClientStateMessage {
    SetGameFrame(GameFrame),
    // FIXME BS NOW: send when placed or when client say hello, with last known window
    SetWindow(Window),
    SetGameSlice(GameSlice),
    SetCity(ClientCity),
    RemoveCity(CityId),
    SetUnit(ClientUnit),
    RemoveUnit(UnitId),
}

#[derive(Error, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TakePlaceRefusedReason {
    #[error("Flag {0} already taken")]
    FlagAlreadyTaken(Flag),
}
