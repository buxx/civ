use core::CorePlugin;

use bevy::prelude::*;
#[cfg(feature = "debug")]
use debug::DebugPlugin;
use ingame::InGamePlugin;
use network::NetworkPlugin;
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
mod menu;
mod network;
mod state;
mod utils;
mod window;

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(window_plugin()),
        StatePlugin::builder().build(),
        NetworkPlugin::default(),
        MenuPlugin,
        CorePlugin,
        InGamePlugin,
    ));

    #[cfg(feature = "debug")]
    {
        app.add_plugins(DebugPlugin)
    }

    app.run();

    Ok(())
}

fn main() {
    entrypoint().unwrap()
}
