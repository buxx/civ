use bevy::prelude::*;
use network::{NetworkPlugin, ServerToClientReceiverResource};
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod menu;
mod network;
mod state;
mod window;

fn display_received(receiver: Res<ServerToClientReceiverResource>) {
    if let Ok(message) = receiver.0.try_recv() {
        info!("received: {:?}", message);
    }
}

#[wasm_bindgen(start)]
fn entrypoint() -> Result<(), JsValue> {
    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin()),
            StatePlugin,
            NetworkPlugin,
            MenuPlugin,
        ))
        .add_systems(Update, display_received)
        .run();

    Ok(())
}

fn main() {
    entrypoint().unwrap()
}
