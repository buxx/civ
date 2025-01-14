pub mod establishment;
use crate::{
    context::Context,
    state::{State, StateError},
};
use clap::{Args, Parser, Subcommand};
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, SendError, Sender};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use thiserror::Error;
use uuid::Uuid;

pub mod city;
pub mod errors;
pub mod status;
pub mod unit;
pub mod window;

pub const FOLLOW_INTERVAL: Duration = Duration::from_millis(250);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    #[clap(long, short, action)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Exit,
    Status,
    Errors,
    TakePlace {
        flag: String,
    },
    Window {
        #[clap(subcommand)]
        subcommand: WindowSubCommand,
    },
    Cities,
    City {
        id: Uuid,
        #[clap(long, short, action)]
        follow: bool,
    },
    Units,
    Unit {
        id: Uuid,
        #[clap(subcommand)]
        subcommand: Option<UnitSubCommand>,
    },
}

#[derive(Debug, Subcommand)]
pub enum UnitSubCommand {
    Detail {
        #[clap(long, short, action)]
        follow: bool,
    },
    Settle {
        city_name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum WindowSubCommand {
    Set {
        start_x: u64,
        start_y: u64,
        end_x: u64,
        end_y: u64,
    },
}

pub struct CommandContext {
    pub context: Context,
    pub state: Arc<RwLock<State>>,
    pub from_server_receiver: Receiver<ServerToClientMessage>,
    pub to_server_sender: Sender<ClientToServerMessage>,
}

impl CommandContext {
    pub fn new(
        context: Context,
        state: Arc<RwLock<State>>,
        from_server_receiver: Receiver<ServerToClientMessage>,
        to_server_sender: Sender<ClientToServerMessage>,
    ) -> Self {
        Self {
            context,
            state,
            from_server_receiver,
            to_server_sender,
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Game state not ready")]
    GameStateNotReady,
    #[error("Unit no more available")]
    UnitNoMoreAvailable,
    #[error("Unexpected closed channel: {0}")]
    Unexpected(#[from] SendError<ClientToServerMessage>),
    #[error("Invalid user input: {0}")]
    InvalidInput(InvalidInputError),
}

#[derive(Error, Debug)]
pub enum InvalidInputError {
    #[error("Invalid flag: {0}")]
    InvalidFlag(String),
}

impl From<StateError> for CommandError {
    fn from(value: StateError) -> Self {
        match value {
            StateError::NotReady => Self::GameStateNotReady,
        }
    }
}
