use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::bridge::native;
use crate::menu::{join::ConnectEvent, state::MenuStateResource};

use super::{ClientToServerReceiverResource, ServerToClientSenderResource};

pub fn connect(
    trigger: Trigger<ConnectEvent>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
    mut state: ResMut<MenuStateResource>,
) {
    let address = trigger.event().0.clone();
    info!("Connecting to {} ...", &address);
    state.connecting = true;
    #[cfg(not(target_arch = "wasm32"))]
    native::connect(
        address,
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    );
}
