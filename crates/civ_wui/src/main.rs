use bevy::prelude::*;
use menu::MenuPlugin;
use state::StatePlugin;
use window::window_plugin;

mod menu;
mod state;
mod window;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin()), StatePlugin, MenuPlugin))
        .run();
}
