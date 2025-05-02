use bevy::prelude::*;
use common::game::slice::ClientCity;
use common::game::{
    slice::ClientUnit, slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame,
};
use common::geo::WorldPoint;
use common::space::window::Window as BaseWindow;
use input::update_last_known_cursor_position;

use crate::core::GameSliceUpdated;
use crate::inject::Injection;
use crate::state::AppState;

pub mod input;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWindowResource>()
            .init_resource::<CameraInitializedResource>()
            .init_resource::<GameFrameResource>()
            .init_resource::<GameSliceResource>()
            .init_resource::<LastKnownCursorPositionResource>()
            .add_systems(Startup, inject)
            // TODO: on state ingame only
            .add_systems(
                Update,
                (update_last_known_cursor_position,).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Default)]
pub struct CameraInitializedResource(pub bool);

#[derive(Resource, Default)]
pub struct LastKnownCursorPositionResource(pub Vec2);

#[derive(Resource, Default)]
pub struct GameFrameResource(pub Option<BaseGameFrame>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameSliceResource(pub Option<BaseGameSlice>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameWindowResource(pub Option<BaseWindow>);

#[derive(Component, Debug)]
pub struct HexTile;

#[derive(Component, Deref, DerefMut)]
pub struct Unit(pub ClientUnit);

#[derive(Component, Deref, DerefMut)]
pub struct City(pub ClientCity);

#[derive(Component, Deref, DerefMut)]
pub struct Point(pub WorldPoint);

pub fn inject(
    mut commands: Commands,
    injection: ResMut<Injection>,
    mut game_slice: ResMut<GameSliceResource>,
) {
    if let Some(game_slice_) = injection.game_slice() {
        game_slice.0 = Some(game_slice_.clone());
        commands.trigger(GameSliceUpdated);
    }
}
