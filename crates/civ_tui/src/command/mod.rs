use crate::{context::Context, state::State};
use clap::{Args, Parser, Subcommand};
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub mod city;
pub mod errors;
pub mod status;
pub mod unit;
pub mod window;

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
    Window {
        #[clap(subcommand)]
        subcommand: WindowSubCommand,
    },
    Cities,
    City {
        id: Uuid,
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
    Detail,
    Settle { city_name: String },
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
