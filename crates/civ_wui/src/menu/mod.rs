pub mod gui;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use camera::spawn_camera;
use despawn::despawn_menu;
use gui::manage_gui;
use manage::manage_menu;
use spawn::spawn_menu;

use crate::state::AppState;

pub mod camera;
pub mod despawn;
pub mod manage;
pub mod spawn;

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(
                Update,
                (manage_menu, manage_gui).run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), despawn_menu);
    }
}
