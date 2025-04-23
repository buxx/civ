use core::CorePlugin;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
use context::{Context, ContextResource};
#[cfg(feature = "debug")]
use debug::DebugPlugin;
use derive_more::Constructor;
use embedded::EmbeddedPlugin;
use ingame::InGamePlugin;
use map::MapPlugin;
use network::{NetworkPlugin, DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT};
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod assets;
mod context;
mod core;
#[cfg(feature = "debug")]
mod debug;
mod embedded;
mod ingame;
mod inject;
mod map;
mod menu;
mod network;
mod state;
mod user;
mod utils;
mod window;

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    let context = Context::new();
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(window_plugin())
            .set(ImagePlugin::default_nearest()),
        StatePlugin::builder().build(),
        EmbeddedPlugin,
        NetworkPlugin::default(),
        MenuPlugin::new(context.clone()),
        CorePlugin,
        MapPlugin,
        InGamePlugin,
    ))
    .insert_resource(ContextResource::new(context));

    #[cfg(feature = "debug")]
    {
        app.add_plugins(DebugPlugin)
    }

    app.run();

    Ok(())
}

// fn network_config() -> network::NetworkConfig {
//     network::NetworkConfig::builder()
//         .server_host(
//             std::option_env!("SERVER_HOST")
//                 .unwrap_or(DEFAULT_SERVER_HOST)
//                 .to_string(),
//         )
//         .server_port(
//             std::option_env!("SERVER_PORT")
//                 .unwrap_or(&format!("{}", DEFAULT_SERVER_PORT))
//                 .parse()
//                 .unwrap(), // TODO
//         )
//         .build()
// }

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, action)]
    embedded_server: bool,
    // /// Path where load and save server snapshot
    // #[arg(short, long)]
    // snapshot: Option<PathBuf>,
    // /// Game frame interval count between two snapshot
    // #[arg(long, default_value = "120000")]
    // snapshot_interval: u64,
    // /// TCP listen address
    // #[arg(short, long, default_value = "127.0.0.1:9876")]
    // tcp_server_address: String,
    // /// WebSocket listen address
    // #[arg(short, long, default_value = "127.0.0.1:9877")]
    // ws_server_address: String,
}

// fn bridge(args: &Args) ->  {}

fn main() {
    // let args = Args::parse();
    // let bridge = bridge(&args);

    // let (client_to_server_sender, client_to_server_receiver) = unbounded();
    // let (server_to_client_sender, server_to_client_receiver) = unbounded();
    // let bridge = DirectBridgeBuilder::new(
    //     client_to_server_sender,
    //     client_to_server_receiver,
    //     server_to_client_sender,
    //     server_to_client_receiver,
    // )
    // .build(context, state, config);
    // civ_server::start(args);

    entrypoint().unwrap()
}
