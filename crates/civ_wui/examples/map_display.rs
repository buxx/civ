use std::path::PathBuf;
use std::thread;

use async_std::channel::{unbounded, Receiver, Sender};
use async_std::task;
use bevy::prelude::*;
use common::network::message::{
    ClientStateMessage, ClientToServerGameMessage, ClientToServerInGameMessage,
    ClientToServerMessage, ServerToClientInGameMessage, ServerToClientMessage,
};
use common::space::window::{DisplayStep, Resolution, SetWindow, Window as SpaceWindow};
use common::world::reader::WorldReader;
use wasm_bindgen::prelude::*;

use civ_wui::core::CorePlugin;
#[cfg(feature = "debug")]
use civ_wui::debug::DebugPlugin;
use civ_wui::ingame::InGamePlugin;
use civ_wui::inject::Injection;
use civ_wui::menu::MenuPlugin;
use civ_wui::network::NetworkPlugin;
use civ_wui::state::{AppState, StatePlugin};
use civ_wui::window::window_plugin;
use common::game::slice::GameSlice;
use common::geo::{ImaginaryWorldPoint, WorldPoint};
use common::world::partial::PartialWorld;
use common::world::{CtxTile, TerrainType, Tile};

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    let world_tiles = vec![
        //
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        //
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        //
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        //
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        //
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        //
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        //
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        //
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        //
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::Plain),
        Tile::new(TerrainType::Plain),
        //
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
        Tile::new(TerrainType::GrassLand),
    ];
    let world = WorldReader::new(PathBuf::from("."), 10, 10, world_tiles);
    let window_start = ImaginaryWorldPoint::new(0, 0);
    let window_end = ImaginaryWorldPoint::new(4, 4);
    let window = SpaceWindow::new(window_start, window_end, DisplayStep::Close);
    let window_tiles = world.window_tiles(&window);
    let partial_world = PartialWorld::new(
        window_start,
        5,
        5,
        window_tiles
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<CtxTile<Tile>>>(),
    );
    let cities = vec![];
    let units = vec![];
    let game_slice = GameSlice::new(partial_world, cities, units);
    // let injection = Injection::builder().game_slice(game_slice).build();

    let (to_server_sender, to_server_receiver): (
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    ) = unbounded();

    let (from_server_sender, from_server_receiver): (
        Sender<ServerToClientMessage>,
        Receiver<ServerToClientMessage>,
    ) = unbounded();

    let to_server_sender_ = to_server_sender.clone();
    let to_server_receiver_ = to_server_receiver.clone();
    let from_server_sender_ = from_server_sender.clone();
    thread::spawn(move || {
        task::block_on(async {
            to_server_sender_
                .send(ClientToServerMessage::Game(
                    ClientToServerGameMessage::InGame(ClientToServerInGameMessage::SetWindow(
                        SetWindow::from_around(
                            &ImaginaryWorldPoint::new(5, 5),
                            &Resolution::new(50, 50),
                        ),
                    )),
                ))
                .await
                .unwrap();
            while let Ok(message) = to_server_receiver_.recv().await {
                if let ClientToServerMessage::Game(ClientToServerGameMessage::InGame(
                    ClientToServerInGameMessage::SetWindow(window),
                )) = message
                {
                    // Intent is to reproduce civ_server::runner::Runner::update_client_window_reflects
                    let window = SpaceWindow::from(window);
                    let window_tiles = world.window_tiles(&window);
                    let partial_world = PartialWorld::new(
                        ImaginaryWorldPoint::new(window.start().x, window.start().y),
                        (window.end().x - window.start().x + 1) as u64,
                        (window.end().y - window.start().y + 1) as u64,
                        window_tiles
                            .into_iter()
                            .map(|t| t.into())
                            .collect::<Vec<CtxTile<Tile>>>(),
                    );
                    let game_slice = GameSlice::new(partial_world, vec![], vec![]);

                    from_server_sender_
                        .send(ServerToClientMessage::InGame(
                            ServerToClientInGameMessage::State(ClientStateMessage::SetWindow(
                                window.clone(),
                            )),
                        ))
                        .await
                        .unwrap();
                    from_server_sender_
                        .send(ServerToClientMessage::InGame(
                            ServerToClientInGameMessage::State(ClientStateMessage::SetGameSlice(
                                game_slice,
                            )),
                        ))
                        .await
                        .unwrap();
                }
            }
        })
    });

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(window_plugin()),
        StatePlugin::builder()
            .init_state(AppState::InGame)
            // .injection(injection)
            .build(),
        NetworkPlugin::builder()
            .to_server_channels((to_server_sender, to_server_receiver))
            .from_server_channels((from_server_sender, from_server_receiver))
            .build(),
        MenuPlugin,
        CorePlugin,
        InGamePlugin,
    ));

    #[cfg(feature = "debug")]
    {
        app.add_plugins(DebugPlugin);
    }

    app.run();

    Ok(())
}

fn main() {
    entrypoint().unwrap()
}
