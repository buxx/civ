use core::CorePlugin;

use bevy::prelude::*;
use network::NetworkPlugin;
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod core;
mod menu;
mod network;
mod state;
mod window;

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin()),
            StatePlugin,
            NetworkPlugin,
            MenuPlugin,
            CorePlugin,
        ))
        .run();

    Ok(())
}

fn main() {
    entrypoint().unwrap()
}
