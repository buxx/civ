use common::{network::message::ClientToServerMessage, space::Window};

use crate::error::PublicError;

use super::CommandContext;

pub fn set(context: CommandContext, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let mut state = context
        .state
        .lock()
        .expect("Assume state is always accessible");

    if !state.connected() {
        state.push_error(PublicError::NotConnected);
        return;
    }

    let window = Window::new(start_x, start_y, end_x, end_y);
    context
        .to_server_sender
        .send(ClientToServerMessage::SetWindow(window))
        .unwrap();
}
