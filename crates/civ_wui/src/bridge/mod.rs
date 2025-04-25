use async_std::channel::{unbounded, Receiver, Sender};

use bevy::prelude::*;
use common::{
    network::{
        message::{
            ClientToServerEstablishmentMessage, ClientToServerGameMessage, ClientToServerMessage,
            ClientToServerNetworkMessage, ServerToClientMessage,
        },
        Client, ServerAddress,
    },
    space::window::Resolution,
};

use crate::{
    core::preferences::PreferencesResource,
    menu::{
        join::{ConnectEvent, JoinEvent, TakePlaceEvent},
        state::MenuStateResource,
    },
    state::ClientIdResource,
};

#[cfg(not(target_arch = "wasm32"))]
mod native;

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
            .add_observer(connect)
            .add_observer(join)
            .add_observer(send_to_server)
            .add_observer(take_place)
            .add_systems(Update, listen_from_server);
    }
}

pub enum BridgeMessage {
    Internal(InternalBridgeMessage),
    Server(ServerToClientMessage),
}

pub enum InternalBridgeMessage {
    ConnectionEstablished(ServerAddress),
}

#[derive(Resource)]
pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

// FIXME BS NOW: replaces usages by commands.trigger
#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<BridgeMessage>);

#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<BridgeMessage>);

#[derive(Event)]
pub struct SendMessageToServerEvent(pub ClientToServerMessage);

#[derive(Event)]
pub struct MessageReceivedFromServerEvent(pub ServerToClientMessage);

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

pub fn join(trigger: Trigger<JoinEvent>, mut commands: Commands, client_id: Res<ClientIdResource>) {
    let player_id = trigger.event().0;
    let client_id = client_id.0;
    info!("Join as player {} and client {}", &player_id, &client_id);
    commands.trigger(SendMessageToServerEvent(ClientToServerMessage::Network(
        ClientToServerNetworkMessage::Hello(
            Client::new(client_id, player_id),
            // FIXME BS NOW
            Resolution::new(1, 1),
        ),
    )));
}

pub fn take_place(trigger: Trigger<TakePlaceEvent>, mut commands: Commands) {
    let flag = trigger.event().0;
    info!("Take place as {}", &flag);
    commands.trigger(SendMessageToServerEvent(ClientToServerMessage::Game(
        ClientToServerGameMessage::Establishment(ClientToServerEstablishmentMessage::TakePlace(
            flag,
        )),
    )));
}

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
                    state.join.player_id = preferences
                        .0
                        .player_id(&address)
                        .cloned()
                        .unwrap_or_default()
                        .to_string();
                    state.join.connected = true;
                    state.connecting = false;
                }
            },
            BridgeMessage::Server(message) => {
                commands.trigger(MessageReceivedFromServerEvent(message));
            }
        }
    }
}
