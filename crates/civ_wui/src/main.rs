use core::CorePlugin;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use debug::DebugPlugin;
use ingame::InGamePlugin;
use map::MapPlugin;
use network::{NetworkPlugin, DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT};
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod assets;
mod core;
#[cfg(feature = "debug")]
mod debug;
mod ingame;
mod inject;
mod map;
mod menu;
mod network;
mod state;
mod utils;
mod window;

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    let mut app = App::new();

    let network_config = network_config();
    app.add_plugins((
        DefaultPlugins
            .set(window_plugin())
            .set(ImagePlugin::default_nearest()),
        StatePlugin::builder().build(),
        NetworkPlugin::builder().config(network_config).build(),
        MenuPlugin,
        CorePlugin,
        MapPlugin,
        InGamePlugin,
    ));

    #[cfg(feature = "debug")]
    {
        app.add_plugins(DebugPlugin)
    }

    app.run();

    Ok(())
}

fn network_config() -> network::NetworkConfig {
    network::NetworkConfig::builder()
        .server_host(
            std::option_env!("SERVER_HOST")
                .unwrap_or(DEFAULT_SERVER_HOST)
                .to_string(),
        )
        .server_port(
            std::option_env!("SERVER_PORT")
                .unwrap_or(&format!("{}", DEFAULT_SERVER_PORT))
                .parse()
                .unwrap(), // TODO
        )
        .build()
}

fn main() {
    // civ_server::start(args);

    entrypoint().unwrap()
}
