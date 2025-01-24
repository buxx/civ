use async_std::channel::{unbounded, Receiver, Sender};
use bevy::prelude::*;
use common::{
    game::PlayerId,
    network::{
        message::{ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage},
        Client, ClientId,
    },
};
use network2::{
    setup_network, ClientToServerReceiverResource, ClientToServerSenderResource,
    ServerToClientReceiverResource, ServerToClientSenderResource,
};
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod menu;
mod network2;
mod state;
mod window;

fn display_received(receiver: Res<ServerToClientReceiverResource>) {
    if let Ok(message) = receiver.0.try_recv() {
        println!("received: {:?}", message);
    }
}

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    let (to_server_sender, to_server_receiver): (
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    ) = unbounded();
    let (from_server_sender, from_server_receiver): (
        Sender<ServerToClientMessage>,
        Receiver<ServerToClientMessage>,
    ) = unbounded();

    to_server_sender
        .send_blocking(ClientToServerMessage::Network(
            ClientToServerNetworkMessage::Hello(Client::new(
                ClientId::default(),
                PlayerId::default(),
            )),
        ))
        .unwrap();

    // while let Ok(m) = from_server_receiver.recv() {
    //     dbg(m);
    // }

    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin()), StatePlugin, MenuPlugin))
        // .add_plugins((MinimalPlugins, LogPlugin::default(), PanicHandlerPlugin))
        .insert_resource(ServerToClientSenderResource(from_server_sender))
        .insert_resource(ServerToClientReceiverResource(from_server_receiver))
        .insert_resource(ClientToServerSenderResource(to_server_sender))
        .insert_resource(ClientToServerReceiverResource(to_server_receiver))
        .add_systems(Startup, setup_network)
        .add_systems(Update, display_received)
        .run();

    Ok(())
}

fn main() {
    entrypoint().unwrap()
}
