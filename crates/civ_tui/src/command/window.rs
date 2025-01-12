use common::{
    network::message::{ClientToServerInGameMessage, ClientToServerMessage},
    space::window::SetWindow,
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

    let window = SetWindow::new(start_x, start_y, end_x, end_y);
    context
        .to_server_sender
        .send(ClientToServerMessage::InGame(
            ClientToServerInGameMessage::SetWindow(window.clone()),
        ))
        .unwrap();
}
