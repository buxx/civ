pub mod state;
use bevy::prelude::*;
use camera::spawn_camera;
use ingame::react_server_message;
use preferences::PreferencesResource;
use window::react_game_window_updated;

use crate::{core::ingame::on_game_window_updated, user::preferences::Preferences};

pub mod camera;
pub mod establishment;
pub mod ingame;
pub mod preferences;
pub mod window;

pub struct CorePlugin;

#[allow(unused)]
#[derive(Component)]
pub struct Menu;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let preferences = PreferencesResource::new(Preferences::from_env().unwrap());

        app.insert_resource(preferences)
            .add_systems(Startup, spawn_camera)
            .add_observer(react_game_window_updated)
            .add_observer(react_server_message)
            .add_observer(on_game_window_updated);
    }
}

// TODO: move
#[derive(Event)]
pub struct GameWindowUpdated;

// TODO: move
#[derive(Event)]
pub struct GameSliceUpdated;
// TODO: move
#[derive(Event)]
pub struct GameSlicePropagated;
