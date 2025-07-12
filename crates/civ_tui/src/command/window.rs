use common::{
    network::message::{
        ClientToServerGameMessage, ClientToServerInGameMessage, ClientToServerMessage,
    },
    space::window::{DisplayStep, Window},
};

use crate::error::PublicError;

use super::CommandContext;

pub fn set(context: CommandContext, start_x: u64, start_y: u64, end_x: u64, end_y: u64) {
    let state = context
        .state
        .read()
        .expect("Assume state is always accessible");

    if !state.connected() {
        println!("{}", PublicError::NotConnected);
        return;
    }

    let window = Window::new(
        (start_x, start_y).into(),
        (end_x, end_y).into(),
        DisplayStep::Close,
    );
    context
        .to_server_sender
        .send(ClientToServerMessage::Game(
            ClientToServerGameMessage::InGame(ClientToServerInGameMessage::SetWindow(window)),
        ))
        .unwrap();
}
