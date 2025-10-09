// use bevy::prelude::*;
// use common::{
//     network::message::{
//         ClientToServerEstablishmentMessage, ClientToServerGameMessage, ClientToServerMessage,
//         ClientToServerNetworkMessage, ServerToClientEstablishmentMessage,
//     },
//     space::window::Resolution,
// };

// #[cfg(target_arch = "wasm32")]
// use common::game::PlayerId;
// #[cfg(target_arch = "wasm32")]
// use std::str::FromStr;
// use std::thread;

// #[cfg(target_arch = "wasm32")]
// use crate::utils::cookies::Cookies;
// use crate::{
//     network::{BridgeResource, ClientToServerSenderResource, EstablishmentMessage},
//     state::{AppState, ClientResource, ServerResource},
// };

// use super::{ConnectEvent, TakePlaceEvent};

// #[allow(unused)]
// pub fn auto_login(mut commands: Commands) {
//     #[cfg(target_arch = "wasm32")]
//     {
//         if Cookies
//             .get_keep_connected()
//             .unwrap_or(Some(false))
//             .unwrap_or(false)
//             && Cookies.get_player_id().unwrap_or(None).is_some()
//         {
//             commands.trigger(ConnectEvent);
//         }
//     }
// }

// pub fn react_connect(
//     _trigger: On<ConnectEvent>,
//     to_server_sender: Res<ClientToServerSenderResource>,
//     player_id_input: Res<PlayerIdInput>,
//     keep_connected_input: Res<KeepConnectedInput>,
//     mut client: ResMut<ClientResource>,
//     mut connecting: ResMut<ConnectingResource>,
//     mut bridge: ResMut<BridgeResource>,
// ) {
//     if player_id_input.0.is_empty() {
//         return;
//     }
//     #[cfg(target_arch = "wasm32")]
//     {
//         Cookies
//             .set_player_id(&PlayerId::from_str(&player_id_input.0).unwrap())
//             .unwrap();
//         Cookies.set_keep_connected(keep_connected_input.0).unwrap();
//         client
//             .0
//             .set_player_id(PlayerId::from_str(&player_id_input.0).unwrap());
//     }
//     connecting.0 = true;

//     // FIXME HERE FAKE embedded server choice (must be user input)
//     // *bridge = DirectBridge::new();
//     thread::spawn(move || {});

//     info!("HELLO");
//     to_server_sender
//         .0
//         .send_blocking(ClientToServerMessage::Network(
//             ClientToServerNetworkMessage::Hello(
//                 client.0,
//                 // FIXME BS NOW
//                 Resolution::new(1, 1),
//             ),
//         ))
//         .unwrap();
// }

// pub fn take_place(
//     _trigger: On<TakePlaceEvent>,
//     to_server_sender: Res<ClientToServerSenderResource>,
//     // flag_input: Res<FlagInput>,
//     mut taking_place: ResMut<TakingPlaceResource>,
//     server: Res<ServerResource>,
// ) {
//     if let Some(flag) = server.flag {
//         taking_place.0 = true;
//         to_server_sender
//             .0
//             .send_blocking(ClientToServerMessage::Game(
//                 ClientToServerGameMessage::Establishment(
//                     ClientToServerEstablishmentMessage::TakePlace(flag),
//                 ),
//             ))
//             .unwrap();
//     }
// }

// pub fn react_establishment(
//     trigger: On<EstablishmentMessage>,
//     mut next_state: ResMut<NextState<AppState>>,
//     mut connecting: ResMut<ConnectingResource>,
//     mut taking_place: ResMut<TakingPlaceResource>,
// ) {
//     match &trigger.event().0 {
//         ServerToClientEstablishmentMessage::ServerResume(_, flag) => {
//             connecting.0 = false;
//             taking_place.0 = false;

//             if flag.is_some() {
//                 next_state.set(AppState::InGame);
//             }
//         }
//         ServerToClientEstablishmentMessage::TakePlaceRefused(_reason) => {
//             // TODO: error message display
//         }
//     }
// }
