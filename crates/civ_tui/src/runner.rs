use std::sync::{Arc, Mutex};

use bon::Builder;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use crossbeam::channel::{Receiver, Sender};

use crate::{context::Context, state::State};

#[derive(Builder)]
pub struct Runner {
    context: Arc<Mutex<Context>>,
    state: Arc<Mutex<State>>,
    from_server_receiver: Receiver<ServerToClientMessage>,
    to_server_sender: Sender<ClientToServerMessage>,
}

impl Runner {
    pub fn run(&mut self) {
        //
    }
}
