use async_std::channel::unbounded;
use bevy::prelude::*;
use civ_server::{bridge::direct::DirectBridgeBuilder, start as start_server, Args as ServerArgs};
use civ_world::config::WorldConfig;
use civ_world::writer::FilesWriter;
use common::network::Client;
use common::utils::Progress;
use common::world::TerrainType;
use std::error::Error;
use std::thread;
use uuid::Uuid;
use world::generator::PatternGenerator;

use civ_gui::bridge::{BridgePlugin, ClientToServerSenderResource, ServerToClientReceiverResource};
use civ_gui::context::Context;
use civ_gui::core::CorePlugin;
use civ_gui::ingame::InGamePlugin;
use civ_gui::map::MapPlugin;
use civ_gui::menu::MenuPlugin;
use civ_gui::state::{AppState, StatePlugin};
use civ_gui::window::window_plugin;

mod world;

fn main() -> Result<(), Box<dyn Error>> {
    // let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // env_logger::init_from_env(env);

    println!("Initialize ...");
    let tmp_path = std::env::temp_dir();
    let game_path = tmp_path.join(Uuid::new_v4().to_string());
    let world_path = game_path.join("world");
    let world_config = WorldConfig::new(world_path.clone(), 10, 10, 10);
    let client = Client::default();
    let server_config = ServerArgs::builder()
        .snapshot_interval(0)
        .tcp_listen_address("".to_string())
        .ws_listen_address("".to_string())
        .build();

    let (client_to_server_sender, client_to_server_receiver) = unbounded();
    let (server_to_client_sender, server_to_client_receiver) = unbounded();
    let (progress_sender, progress_receiver) = unbounded();

    // Generate world
    println!("Generate world");
    let generator = PatternGenerator::new([TerrainType::Plain, TerrainType::GrassLand].to_vec());
    let writer = FilesWriter::new(world_path.clone());
    let world = world_config.into();
    civ_world::run()
        // TODO: Choose generator type by arg
        .generator(generator)
        .target(&world_path)
        .world(&world)
        .writer(&writer)
        .call()?;

    // Start server
    println!("Start server");
    thread::spawn(move || {
        let bridge =
            DirectBridgeBuilder::new(client, client_to_server_receiver, server_to_client_sender);
        let _ = start_server()
            .args(server_config)
            .bridge_builder(&bridge)
            .progress(progress_sender)
            .call();
    });

    // Wait server ready
    println!("Wait server is ready ...");
    while let Ok(message) = progress_receiver.recv_blocking() {
        if let Progress::Finished = message {
            break;
        }
    }
    println!("Server ready, start GUI");
    let to_server_sender = ClientToServerSenderResource(client_to_server_sender);
    let from_server_receiver = ServerToClientReceiverResource(server_to_client_receiver);

    // FIXME: inject things like ClientToServerEstablishmentMessage

    let mut app = App::new();
    let context = Context::new();
    app.add_plugins((
        DefaultPlugins
            .set(window_plugin())
            .set(ImagePlugin::default_nearest()),
        StatePlugin::builder()
            .init_state(AppState::InGame)
            // .injection(injection)
            .build(),
        BridgePlugin::builder()
            .to_server_sender(to_server_sender)
            .from_server_receiver(from_server_receiver)
            .build(),
        MenuPlugin::new(context.clone()),
        CorePlugin,
        InGamePlugin,
        MapPlugin,
    ));

    #[cfg(feature = "debug")]
    {
        app.add_plugins(DebugPlugin);
    }

    app.run();

    Ok(())
}
