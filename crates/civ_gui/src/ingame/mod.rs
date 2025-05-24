pub mod menu;
use bevy::prelude::*;
use bon::Builder;
use common::game::{slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame};
use common::geo::WorldPoint;
use common::space::window::Window as BaseWindow;
use hexx::Hex;
use input::menu::on_try_menu;
use input::select::on_try_select;
use input::{on_click, update_last_known_cursor_position};
use menu::MenuResource;
use selected::{on_select_updated, SelectedResource};

use crate::state::AppState;

pub mod input;
pub mod selected;

#[derive(Builder)]
pub struct InGamePlugin {
    game_slice: Option<GameSliceResource>,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWindowResource>()
            .init_resource::<CameraInitializedResource>()
            .init_resource::<GameFrameResource>()
            .init_resource::<LastKnownCursorPositionResource>()
            .init_resource::<SelectedResource>()
            .init_resource::<MenuResource>()
            .insert_resource(
                self.game_slice
                    .as_ref()
                    .unwrap_or(&GameSliceResource(None))
                    .clone(),
            )
            .add_systems(
                Update,
                (update_last_known_cursor_position,).run_if(in_state(AppState::InGame)),
            )
            .add_observer(on_click)
            .add_observer(on_try_select)
            .add_observer(on_try_menu)
            .add_observer(on_select_updated);
    }
}

#[derive(Resource, Default)]
pub struct CameraInitializedResource(pub bool);

#[derive(Resource, Default)]
pub struct LastKnownCursorPositionResource(pub Vec2);

#[derive(Resource, Default)]
pub struct GameFrameResource(pub Option<BaseGameFrame>);

#[derive(Resource, Default, Deref, DerefMut, Clone)]
pub struct GameSliceResource(pub Option<BaseGameSlice>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameWindowResource(pub Option<BaseWindow>);

#[derive(Component, Debug, Clone, Copy)]
pub struct HexTile;

#[derive(Component, Debug, Clone, Copy)]
pub struct HexUnit;

#[derive(Component, Debug, Clone, Copy)]
pub struct HexCity;

#[derive(Component, Deref, DerefMut)]
pub struct Point(pub WorldPoint);

#[derive(Debug, Event)]
pub struct TrySelect(Hex);

#[derive(Debug, Event)]
pub struct TryMenu(Hex);
