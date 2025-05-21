use core::CorePlugin;

use bevy::prelude::*;
use bridge::BridgePlugin;
use context::{Context, ContextResource};
#[cfg(feature = "debug")]
use debug::DebugPlugin;
use embedded::EmbeddedPlugin;
use ingame::InGamePlugin;
use map::MapPlugin;
use user::UserPlugin;
use wasm_bindgen::prelude::*;

use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod assets;
mod atlas;
mod bridge;
mod context;
mod core;
#[cfg(feature = "debug")]
mod debug;
mod embedded;
mod ingame;
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
        BridgePlugin::builder().build(),
        // NetworkPlugin::default(),
        UserPlugin,
        MenuPlugin::new(context.clone()),
        CorePlugin,
        MapPlugin,
        InGamePlugin::builder().build(),
    ))
    .insert_resource(ContextResource::new(context));

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
