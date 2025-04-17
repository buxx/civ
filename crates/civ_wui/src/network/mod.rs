use async_std::channel::{unbounded, Receiver, Sender};
use bevy::prelude::*;
use bon::Builder;
use common::network::message::{
    ClientToServerMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
    ServerToClientMessage,
};

#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use native::setup_network;
#[cfg(target_arch = "wasm32")]
use wasm::setup_network;

#[derive(Resource)]
pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<ServerToClientMessage>);

#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<ServerToClientMessage>);

#[derive(Resource, Deref)]
pub struct NetworkConfigResource(NetworkConfig);

#[derive(Event)]
pub struct ServerMessage(pub ServerToClientMessage);

#[derive(Event)]
pub struct EstablishmentMessage(pub ServerToClientEstablishmentMessage);

#[derive(Event)]
pub struct InGameMessage(pub ServerToClientInGameMessage);

pub const DEFAULT_SERVER_HOST: &str = "127.0.0.1";
#[cfg(target_arch = "wasm32")]
pub const DEFAULT_SERVER_PORT: u16 = 9877;
#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_SERVER_PORT: u16 = 9876;

#[derive(Debug, Builder, Clone)]
pub struct NetworkConfig {
    pub server_host: String,
    pub server_port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server_host: DEFAULT_SERVER_HOST.to_string(),
            server_port: DEFAULT_SERVER_PORT,
        }
    }
}

#[derive(Default, Builder)]
pub struct NetworkPlugin {
    config: NetworkConfig,
    to_server_channels: Option<(
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    )>,
    from_server_channels: Option<(
        Sender<ServerToClientMessage>,
        Receiver<ServerToClientMessage>,
    )>,
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let (to_server_sender, to_server_receiver): (
            Sender<ClientToServerMessage>,
            Receiver<ClientToServerMessage>,
        ) = self.to_server_channels.clone().unwrap_or(unbounded());

        let (from_server_sender, from_server_receiver): (
            Sender<ServerToClientMessage>,
            Receiver<ServerToClientMessage>,
        ) = self.from_server_channels.clone().unwrap_or(unbounded());

        app.insert_resource(NetworkConfigResource(self.config.clone()))
            .insert_resource(ServerToClientSenderResource(from_server_sender))
            .insert_resource(ServerToClientReceiverResource(from_server_receiver))
            .insert_resource(ClientToServerSenderResource(to_server_sender))
            .insert_resource(ClientToServerReceiverResource(to_server_receiver))
            .add_systems(Startup, setup_network)
            .add_systems(Update, react_incoming);
    }
}

fn react_incoming(mut commands: Commands, receiver: Res<ServerToClientReceiverResource>) {
    while let Ok(message) = receiver.0.try_recv() {
        commands.trigger(ServerMessage(message));
    }
}
