use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use bevy_async_task::AsyncTaskRunner;

#[cfg(not(target_arch = "wasm32"))]
use crate::bridge::native;
#[cfg(target_arch = "wasm32")]
use crate::bridge::wasm;
use crate::menu::{join::ConnectEvent, state::MenuStateResource};

use super::{ClientToServerReceiverResource, ServerToClientSenderResource};

#[cfg(not(target_arch = "wasm32"))]
pub fn connect(
    trigger: Trigger<ConnectEvent>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
    mut state: ResMut<MenuStateResource>,
) {
    let address = trigger.event().0.clone();
    info!("Connecting to {} ...", &address);
    state.connecting = true;
    native::connect(
        address,
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn connect(
    trigger: Trigger<ConnectEvent>,
    mut task_runner: AsyncTaskRunner<'_, ()>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
    mut state: ResMut<MenuStateResource>,
) {
    let address = trigger.event().0.clone();
    info!("Connecting to {} ...", &address);
    state.connecting = true;
    wasm::connect(
        task_runner,
        address,
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    );
}
