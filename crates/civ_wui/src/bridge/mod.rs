use async_std::channel::{unbounded, Receiver, Sender};

use bevy::prelude::*;
use civ_world::WorldGeneratorError;
use common::{
    network::{
        message::{ClientToServerMessage, ServerToClientMessage},
        ServerAddress,
    },
    utils::Progress,
    world::reader::WorldReaderError,
};

use crate::{
    core::preferences::PreferencesResource,
    menu::state::{MenuState, MenuStateResource},
    state::AppState,
    user::preferences::Preferences,
};

mod connect;
mod join;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub mod single;
mod take_place;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[derive(Default)]
pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        let (to_server_sender, to_server_receiver): (
            Sender<ClientToServerMessage>,
            Receiver<ClientToServerMessage>,
        ) = unbounded();

        let (from_server_sender, from_server_receiver): (
            Sender<BridgeMessage>,
            Receiver<BridgeMessage>,
        ) = unbounded();

        app.insert_resource(ServerToClientSenderResource(from_server_sender))
            .insert_resource(ServerToClientReceiverResource(from_server_receiver))
            .insert_resource(ClientToServerSenderResource(to_server_sender))
            .insert_resource(ClientToServerReceiverResource(to_server_receiver))
            .insert_resource(WorldGenerationProgressReceiverResource(None))
            .insert_resource(StartEmbeddedServerReceiverResource(None))
            .add_observer(connect::connect)
            .add_observer(join::join)
            .add_observer(send_to_server)
            .add_observer(take_place::take_place)
            .add_observer(single::start_single)
            .add_observer(single::start_embedded_server)
            .add_observer(single::join_embedded_server)
            .add_systems(
                Update,
                single::listen_world_generation_progress.run_if(in_state(AppState::Menu)),
            )
            .add_systems(
                Update,
                single::listen_start_embedded_server_progress.run_if(in_state(AppState::Menu)),
            )
            .add_systems(Update, listen_from_server);
    }
}

pub enum BridgeMessage {
    Internal(InternalBridgeMessage),
    Server(ServerToClientMessage),
}

impl From<ServerToClientMessage> for BridgeMessage {
    fn from(value: ServerToClientMessage) -> Self {
        BridgeMessage::Server(value)
    }
}

pub enum InternalBridgeMessage {
    ConnectionEstablished(ServerAddress),
}

#[derive(Resource)]
pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<BridgeMessage>);

#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<BridgeMessage>);

#[derive(Resource)]
pub struct WorldGenerationProgressReceiverResource(
    pub Option<Receiver<Progress<WorldGeneratorError>>>,
);

#[derive(Resource)]
pub struct StartEmbeddedServerReceiverResource(pub Option<Receiver<Progress<WorldReaderError>>>);

#[derive(Event)]
pub struct StartEmbeddedServer;

#[derive(Event)]
pub struct EmbeddedServerReady;

#[derive(Event)]
pub struct SendMessageToServerEvent(pub ClientToServerMessage);

#[derive(Event)]
pub struct MessageReceivedFromServerEvent(pub ServerToClientMessage);

fn send_to_server(
    trigger: Trigger<SendMessageToServerEvent>,
    to_server_sender: Res<ClientToServerSenderResource>,
) {
    to_server_sender
        .0
        .send_blocking(trigger.event().0.clone())
        .unwrap();
}

fn listen_from_server(
    mut commands: Commands,
    mut state: ResMut<MenuStateResource>,
    preferences: Res<PreferencesResource>,
    receiver: Res<ServerToClientReceiverResource>,
) {
    while let Ok(message) = receiver.0.try_recv() {
        match message {
            BridgeMessage::Internal(message) => match message {
                InternalBridgeMessage::ConnectionEstablished(address) => {
                    connection_established(&mut state, &preferences, address);
                }
            },
            BridgeMessage::Server(message) => {
                commands.trigger(MessageReceivedFromServerEvent(message));
            }
        }
    }
}

fn connection_established(
    state: &mut MenuState,
    preferences: &Preferences,
    address: ServerAddress,
) {
    state.join.player_id = preferences
        .player_id(&address)
        .cloned()
        .unwrap_or_default()
        .to_string();
    state.join.keep_connected = *preferences.keep_connected(&address).unwrap_or(&false);
    state.join.connected = true;
    state.connecting = false;
}
