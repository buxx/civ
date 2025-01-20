use bevy::{prelude::*, window::WindowResolution};
use menu::{
    camera::CameraPlugin as MenuCameraPlugin, despawn::despawn_menu, manage::manage_menu,
    spawn::spawn_menu,
};
use state::{AppState, InGame};

pub mod menu;
pub mod state;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "civ".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(800.0, 600.0),
                    ..default()
                }),
                ..default()
            }),
            MenuCameraPlugin, // TODO: plugin for entire menu because spawn when enter AppState::Menu, etc ...
        ))
        .init_state::<AppState>()
        .add_sub_state::<InGame>()
        .add_systems(OnEnter(AppState::Menu), spawn_menu)
        .add_systems(Update, manage_menu.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), despawn_menu)
        .run();
}
