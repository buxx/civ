pub mod status;
pub mod window;
use std::sync::{Arc, Mutex};

use clap::{Args, Parser, Subcommand};
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};

use crate::{context::Context, state::State};

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
    Window {
        #[clap(subcommand)]
        subcommand: WindowSubCommand,
    },
}
#[derive(Debug, Subcommand)]
pub enum WindowSubCommand {
    Set {
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
    },
}

pub struct CommandContext {
    pub context: Arc<Mutex<Context>>,
    pub state: Arc<Mutex<State>>,
    pub from_server_receiver: Receiver<ServerToClientMessage>,
    pub to_server_sender: Sender<ClientToServerMessage>,
}

impl CommandContext {
    pub fn new(
        context: Arc<Mutex<Context>>,
        state: Arc<Mutex<State>>,
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
