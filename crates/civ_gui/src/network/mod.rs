// use async_std::channel::{unbounded, Receiver, Sender};
// use bevy::prelude::*;
// use bon::Builder;
// use common::network::message::{
//     ClientToServerMessage, ServerToClientEstablishmentMessage, ServerToClientInGameMessage,
//     ServerToClientMessage,
// };

// #[cfg(not(target_arch = "wasm32"))]
// pub mod native;
// #[cfg(target_arch = "wasm32")]
// pub mod wasm;

// use derive_more::Constructor;
// #[cfg(not(target_arch = "wasm32"))]
// use native::connect;
// use serde::{Deserialize, Serialize};
// #[cfg(target_arch = "wasm32")]
// use wasm::setup_network;

// use crate::{menu::ConnectingResource, state::ServerResource};

// #[derive(Resource, Deref, DerefMut, Default)]
// pub struct BridgeResource(Option<Box<dyn Bridge>>);

// #[derive(Resource)]
// pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

// #[derive(Resource)]
// pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

// #[derive(Resource)]
// pub struct ServerToClientReceiverResource(pub Receiver<BridgeMessage>);

// #[derive(Resource)]
// pub struct ServerToClientSenderResource(pub Sender<BridgeMessage>);

// // #[derive(Resource, Deref)]
// // pub struct NetworkConfigResource(NetworkConfig);

// #[derive(Event, Deref, Constructor)]
// pub struct JoinServer(pub NetworkConfig);

// #[derive(Event)]
// pub struct ServerMessage(pub ServerToClientMessage);

// #[derive(Event)]
// pub struct EstablishmentMessage(pub ServerToClientEstablishmentMessage);

// #[derive(Event)]
// pub struct InGameMessage(pub ServerToClientInGameMessage);

// pub const DEFAULT_SERVER_HOST: &str = "127.0.0.1";
// #[cfg(target_arch = "wasm32")]
// pub const DEFAULT_SERVER_PORT: u16 = 9877;
// #[cfg(not(target_arch = "wasm32"))]
// pub const DEFAULT_SERVER_PORT: u16 = 9876;

// pub trait Bridge: Sync + Send {}

// #[derive(
//     Debug, Deref, DerefMut, Constructor, Clone, Hash, PartialEq, Eq, Serialize, Deserialize,
// )]
// pub struct ServerAddress(pub String);

// // #[derive(Debug, Builder, Clone)]
// // pub struct NetworkConfig {
// //     pub server_address: ServerAddress,
// // }

// // pub enum BridgeMessage {
// //     Internal(InternalBridgeMessage),
// //     Server(ServerToClientMessage),
// // }

// // pub enum InternalBridgeMessage {
// //     ConnectionEstablished(ServerAddress),
// // }

// #[derive(Default, Builder)]
// pub struct NetworkPlugin {
//     to_server_channels: Option<(
//         Sender<ClientToServerMessage>,
//         Receiver<ClientToServerMessage>,
//     )>,
//     from_server_channels: Option<(Sender<BridgeMessage>, Receiver<BridgeMessage>)>,
// }

// impl Plugin for NetworkPlugin {
//     fn build(&self, app: &mut App) {
//         let (to_server_sender, to_server_receiver): (
//             Sender<ClientToServerMessage>,
//             Receiver<ClientToServerMessage>,
//         ) = self.to_server_channels.clone().unwrap_or(unbounded());

//         let (from_server_sender, from_server_receiver): (
//             Sender<BridgeMessage>,
//             Receiver<BridgeMessage>,
//         ) = self.from_server_channels.clone().unwrap_or(unbounded());

//         app.init_resource::<BridgeResource>()
//             // .insert_resource(NetworkConfigResource(self.config.clone()))
//             .insert_resource(ServerToClientSenderResource(from_server_sender))
//             .insert_resource(ServerToClientReceiverResource(from_server_receiver))
//             .insert_resource(ClientToServerSenderResource(to_server_sender))
//             .insert_resource(ClientToServerReceiverResource(to_server_receiver))
//             .add_observer(connect)
//             .add_systems(Update, react_bridge);
//     }
// }
