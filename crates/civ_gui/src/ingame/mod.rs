mod animation;
use bevy::prelude::*;
use bevy_egui::egui;
use bon::Builder;
use common::game::{slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame};
use common::geo::WorldPoint;
use common::space::window::Window as BaseWindow;
use input::menu::on_try_menu;
use input::select::on_try_select;
use input::{on_click, update_last_known_cursor_position};
use interact::unit::settle::on_setup_settle;
use selected::{on_select_updated, SelectedResource};

use crate::ingame::animation::{fade_animations, sprite_sheet_animations};
use crate::ingame::input::info::on_try_tile_info;
use crate::ingame::interact::unit::info::UnitInfoResource;
use crate::ingame::interact::unit::settle::SettleCityNameResource;
use crate::ingame::menu::city::CityMenuResource;
use crate::ingame::menu::info::TileInfoMenuResource;
use crate::ingame::menu::unit::UnitMenuResource;
use crate::ingame::selected::select_on_game_slice_propagated;
use crate::state::AppState;
use crate::utils::assets::Progress;
use crate::{add_city_component, add_tile_component, add_unit_component};

pub mod input;
pub mod interact;
pub mod menu;
pub mod selected;

pub const EGUI_DISPLAY_FACTOR: f32 = 1.5;

#[derive(Builder)]
pub struct InGamePlugin {
    game_slice: Option<GameSliceResource>,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWindowResource>()
            .init_resource::<GameFrameResource>()
            .init_resource::<LastKnownCursorPositionResource>()
            .init_resource::<SelectedResource>()
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
            .add_systems(
                Update,
                (fade_animations,).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (sprite_sheet_animations,).run_if(in_state(AppState::InGame)),
            )
            .add_observer(on_click)
            .add_observer(on_try_select)
            .add_observer(on_try_menu)
            .add_observer(on_try_tile_info)
            .add_observer(on_setup_settle)
            .add_observer(on_select_updated)
            .add_observer(update_progresses)
            .add_observer(select_on_game_slice_propagated);

        add_city_component!(app, CityMenuResource);
        add_unit_component!(app, UnitMenuResource);
        add_tile_component!(app, TileInfoMenuResource);
        add_unit_component!(app, SettleCityNameResource);
        add_unit_component!(app, UnitInfoResource);
    }
}

#[derive(Resource, Default)]
pub struct LastKnownCursorPositionResource(pub Vec2);

#[derive(Resource, Default)]
pub struct GameFrameResource(pub Option<BaseGameFrame>);

#[derive(Event)]
pub struct GameFrameUpdated(pub BaseGameFrame);

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

// #[derive(Component, Deref, DerefMut)]
// pub struct Point(pub WorldPoint);

#[derive(Debug, Event)]
pub struct TrySelect(WorldPoint);

#[derive(Debug, Event)]
pub struct TryTileInfo(WorldPoint);

#[derive(Debug, Event)]
pub struct TryMenu(WorldPoint);

pub trait DrawUiComponent {
    fn draw(
        &mut self,
        egui: &egui::Context,
        window: &Window,
        commands: &mut Commands,
        frame: BaseGameFrame,
    ) -> bool;
}

fn update_progresses(
    trigger: On<GameFrameUpdated>,
    mut query: Query<(&mut Progress, &mut Text2d)>,
) {
    let frame = trigger.event().0;
    for (mut progress, mut text) in &mut query {
        let total = progress.end.0 - progress.start.0;
        let current = (frame.0 - progress.start.0) as f32 / total as f32;
        progress.current = current;
        text.0 = format!("{:.0}%", current * 100.);
    }
}
