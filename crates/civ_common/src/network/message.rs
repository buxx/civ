use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    game::{
        city::{CityExploitation, CityProduction},
        nation::flag::Flag,
        server::ServerResume,
        slice::{ClientCity, ClientUnit, GameSlice},
        GameFrame,
    },
    space::window::{SetWindow, Window},
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
    Hello(Client),
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
    Establishment(ServerToClientEstablishmentMessage),
    InGame(ServerToClientInGameMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServerToClientEstablishmentMessage {
    ServerResume(ServerResume, Option<Flag>), // None flag mean player not placed
    // FIXME BS NOW: return that instead generic error
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
    SetWindow(Window),
    SetGameSlice(GameSlice),
    SetCity(ClientCity),
    RemoveCity(Uuid),
    SetUnit(ClientUnit),
    RemoveUnit(Uuid),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TakePlaceRefusedReason {
    FlagAlreadyTaken(Flag),
}
