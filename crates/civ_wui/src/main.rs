use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use network::start_websocket;
use state::StatePlugin;
use window::window_plugin;

mod menu;
mod network;
mod state;
mod window;

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    start_websocket()?;

    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin()), StatePlugin, MenuPlugin))
        .run();

    Ok(())
}

fn main() {}
