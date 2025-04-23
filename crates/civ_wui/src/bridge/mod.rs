use async_std::channel::{unbounded, Receiver, Sender};

use bevy::prelude::*;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};

use crate::network::ServerAddress;

#[derive(Default)]
pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observer);
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

#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<BridgeMessage>);

#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<BridgeMessage>);

#[derive(Event)]
pub struct SendToServerEvent(pub ClientToServerMessage);
