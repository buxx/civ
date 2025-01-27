pub mod gui;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use despawn::despawn_menu;
use gui::{manage_gui, FlagInput, KeepConnectedInput, PlayerIdInput};
use manage::{auto_login, react_connect, react_establishment, react_take_place};
use spawn::spawn_menu;

use crate::state::AppState;

pub mod despawn;
pub mod manage;
pub mod spawn;

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(PlayerIdInput::from_cookies())
            .insert_resource(KeepConnectedInput::from_cookies())
            .init_resource::<FlagInput>()
            .init_resource::<Connecting>()
            .init_resource::<TakingPlace>()
            .add_systems(OnEnter(AppState::Menu), (spawn_menu, auto_login))
            .add_systems(Update, manage_gui.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_menu)
            .add_observer(react_connect)
            .add_observer(react_take_place)
            .add_observer(react_establishment);
    }
}

#[derive(Event)]
pub struct Connect;

#[derive(Event)]
pub struct TakePlace;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Connecting(pub bool);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TakingPlace(pub bool);
