// use std::path::PathBuf;
// use std::thread;

// use async_std::channel::{unbounded, Receiver, Sender};
// use async_std::task;
// use bevy::prelude::*;
// use civ_gui::bridge::{
//     BridgeMessage, BridgePlugin, ClientToServerSenderResource, ServerToClientReceiverResource,
// };
// use civ_gui::context::Context;
// use civ_gui::map::MapPlugin;
// use civ_server::world::reader::WorldReader;
// use common::world::slice::Slice;
// use wasm_bindgen::prelude::*;

// use civ_gui::core::{CorePlugin, GameSliceUpdated};
// use civ_gui::ingame::{GameSliceResource, InGamePlugin};
// use civ_gui::menu::MenuPlugin;
// use civ_gui::state::{AppState, StatePlugin};
// use civ_gui::window::window_plugin;
// use common::game::slice::GameSlice;
// use common::geo::ImaginaryWorldPoint;
// use common::network::message::{
//     ClientStateMessage, ClientToServerGameMessage, ClientToServerInGameMessage,
//     ClientToServerMessage, ServerToClientInGameMessage, ServerToClientMessage,
// };
// use common::space::window::{DisplayStep, Resolution, Window as SpaceWindow};
// use common::world::{TerrainType, Tile};

// #[cfg(feature = "debug")]
// use civ_gui::debug::DebugPlugin;

// #[wasm_bindgen(start)]
// fn entrypoint() -> Result<(), JsValue> {
//     let world_tiles = vec![
//         //
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         //
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         //
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         //
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         //
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         //
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         //
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         //
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         //
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::Plain),
//         Tile::new(TerrainType::Plain),
//         //
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//         Tile::new(TerrainType::GrassLand),
//     ];
//     let world = WorldReader::new(PathBuf::from("."), 10, 10, world_tiles);
//     let window_start = ImaginaryWorldPoint::new(0, 0);
//     let window_width = 4;
//     let window_height = 4;
//     let window_end = ImaginaryWorldPoint::new(
//         window_start.x + window_width,
//         window_start.y + window_height,
//     );
//     let window = SpaceWindow::new(window_start, window_end, DisplayStep::Close);
//     let tiles = world.slice(&window);
//     let cities = Slice::zero();
//     let units = Slice::zero();
//     let game_slice = GameSlice::new(
//         window_start,
//         window_width as u64,
//         window_height as u64,
//         tiles,
//         cities,
//         units,
//     );

//     let (to_server_sender, to_server_receiver): (
//         Sender<ClientToServerMessage>,
//         Receiver<ClientToServerMessage>,
//     ) = unbounded();

//     let (from_server_sender, from_server_receiver): (
//         Sender<ServerToClientMessage>,
//         Receiver<ServerToClientMessage>,
//     ) = unbounded();

//     let to_server_sender_ = to_server_sender.clone();
//     let to_server_receiver_ = to_server_receiver.clone();
//     let from_server_sender_ = from_server_sender.clone();
//     thread::spawn(move || {
//         task::block_on(async {
//             to_server_sender_
//                 .send(ClientToServerMessage::Game(
//                     ClientToServerGameMessage::InGame(ClientToServerInGameMessage::SetWindow(
//                         SpaceWindow::from_around(
//                             &ImaginaryWorldPoint::new(5, 5),
//                             &Resolution::new(50, 50),
//                         ),
//                     )),
//                 ))
//                 .await
//                 .unwrap();
//             while let Ok(message) = to_server_receiver_.recv().await {
//                 if let ClientToServerMessage::Game(ClientToServerGameMessage::InGame(
//                     ClientToServerInGameMessage::SetWindow(window),
//                 )) = message
//                 {
//                     // Intent is to reproduce civ_server::runner::Runner::update_client_window_reflects
//                     let window_tiles = world.slice(&window);
//                     let origin = ImaginaryWorldPoint::new(window.start().x, window.start().y);
//                     let partial_world = Slice::new(
//                         origin,
//                         (window.end().x - window.start().x + 1) as u64,
//                         (window.end().y - window.start().y + 1) as u64,
//                         window_tiles,
//                     );
//                     let game_slice = GameSlice::new(origin, width, height, tiles, cities, units);

//                     from_server_sender_
//                         .send(ServerToClientMessage::InGame(
//                             ServerToClientInGameMessage::State(ClientStateMessage::SetWindow(
//                                 window,
//                             )),
//                         ))
//                         .await
//                         .unwrap();
//                     from_server_sender_
//                         .send(ServerToClientMessage::InGame(
//                             ServerToClientInGameMessage::State(ClientStateMessage::SetGameSlice(
//                                 game_slice,
//                             )),
//                         ))
//                         .await
//                         .unwrap();
//                 }
//             }
//         })
//     });

//     let (from_server_sender_proxy, from_server_receiver_proxy) = unbounded();
//     thread::spawn(move || {
//         while let Ok(message) = from_server_receiver.recv_blocking() {
//             from_server_sender_proxy
//                 .send_blocking(BridgeMessage::Server(message))
//                 .unwrap();
//         }
//     });

//     let init = |mut commands: Commands| {
//         commands.trigger(GameSliceUpdated);
//     };

//     let mut app = App::new();
//     let context = Context::new();
//     app.add_plugins((
//         DefaultPlugins
//             .set(window_plugin())
//             .set(ImagePlugin::default_nearest()),
//         StatePlugin::builder().init_state(AppState::InGame).build(),
//         BridgePlugin::builder()
//             .to_server_sender(ClientToServerSenderResource(to_server_sender))
//             .from_server_receiver(ServerToClientReceiverResource(from_server_receiver_proxy))
//             .build(),
//         MenuPlugin::new(context.clone()),
//         CorePlugin,
//         InGamePlugin::builder()
//             .game_slice(GameSliceResource(Some(game_slice)))
//             .build(),
//         MapPlugin,
//     ))
//     .add_systems(Startup, init);

//     #[cfg(feature = "debug")]
//     {
//         app.add_plugins(DebugPlugin);
//     }

//     app.run();

//     Ok(())
// }

// fn main() {
//     entrypoint().unwrap()
// }

fn main() {}
