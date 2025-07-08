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
    space::window::{Resolution, SetWindow, Window},
};

use super::Client;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NotificationLevel {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServerMessage {
    Network(ClientToServerNetworkMessage),
    Game(ClientToServerGameMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServerNetworkMessage {
    Hello(Client, Resolution),
    Goodbye,
}

impl From<ClientToServerNetworkMessage> for ClientToServerMessage {
    fn from(value: ClientToServerNetworkMessage) -> Self {
        Self::Network(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServerGameMessage {
    Establishment(ClientToServerEstablishmentMessage),
    InGame(ClientToServerInGameMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServerEstablishmentMessage {
    TakePlace(Flag, Resolution),
}

impl From<ClientToServerEstablishmentMessage> for ClientToServerGameMessage {
    fn from(value: ClientToServerEstablishmentMessage) -> Self {
        ClientToServerGameMessage::Establishment(value)
    }
}

impl From<ClientToServerEstablishmentMessage> for ClientToServerMessage {
    fn from(value: ClientToServerEstablishmentMessage) -> Self {
        ClientToServerMessage::Game(ClientToServerGameMessage::Establishment(value))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServerInGameMessage {
    /// Client moved its window
    SetWindow(SetWindow),
    Unit(UnitId, ClientToServerUnitMessage),
    City(CityId, ClientToServerCityMessage),
}

impl From<ClientToServerInGameMessage> for ClientToServerGameMessage {
    fn from(value: ClientToServerInGameMessage) -> Self {
        ClientToServerGameMessage::InGame(value)
    }
}

impl From<ClientToServerInGameMessage> for ClientToServerMessage {
    fn from(value: ClientToServerInGameMessage) -> Self {
        ClientToServerMessage::Game(ClientToServerGameMessage::InGame(value))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientToServerUnitMessage {
    Settle(String), // CityName
    CancelCurrentTask,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
