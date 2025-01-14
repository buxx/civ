use common::network::{
    message::{ClientToServerGameMessage, ServerToClientMessage},
    Client,
};
use crossbeam::channel::{Receiver, Sender};
use uuid::Uuid;

pub mod context;
pub mod effect;
pub mod game;
pub mod network;
pub mod reflect;
pub mod request;
pub mod runner;
pub mod state;
pub mod task;
pub mod utils;
pub mod world;

pub type FromClientsChannels = (
    Sender<(Client, ClientToServerGameMessage)>,
    Receiver<(Client, ClientToServerGameMessage)>,
);
pub type ToClientsChannels = (
    Sender<(Uuid, ServerToClientMessage)>,
    Receiver<(Uuid, ServerToClientMessage)>,
);
