use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiPlugin};

use derive_more::Constructor;
use despawn::despawn_menu;
use spawn::spawn_menu;
use state::{MenuState, MenuStateResource};
use switch::switch;

use crate::{context::Context, state::AppState};

pub mod despawn;
pub mod gui;
pub mod join;
pub mod manage;
pub mod root;
pub mod single;
pub mod spawn;
pub mod state;
pub mod switch;

#[derive(Debug, Constructor)]
pub struct MenuPlugin {
    context: Context,
}

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        let state = MenuStateResource(MenuState::from_context(&self.context));

        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .insert_resource(state)
        .add_systems(OnEnter(AppState::Menu), spawn_menu)
        .add_systems(EguiContextPass, gui::gui.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), despawn_menu)
        .add_observer(switch);
    }
}
