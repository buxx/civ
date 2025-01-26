pub mod slice;
use bevy::prelude::*;
use common::game::{slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame};
use common::space::window::Window as BaseWindow;
use slice::react_game_slice_updated;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWindow>()
            .init_resource::<GameFrame>()
            .init_resource::<GameSlice>()
            .add_observer(react_game_slice_updated);
    }
}

#[derive(Resource, Default)]
pub struct GameFrame(pub Option<BaseGameFrame>);

#[derive(Resource, Default)]
pub struct GameSlice(pub Option<BaseGameSlice>);

#[derive(Resource, Default)]
pub struct GameWindow(pub Option<BaseWindow>);

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct City;
