use bevy::prelude::*;
use bon::Builder;
use common::game::{slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame};
use common::geo::WorldPoint;
use common::space::window::Window as BaseWindow;
use hexx::Hex;
use input::menu::on_try_menu;
use input::select::on_try_select;
use input::{on_click, update_last_known_cursor_position};
use menu::draw::draw_menu;
use menu::{on_menu_effect, MenuResource};
use selected::{on_select_updated, SelectedResource};

use crate::state::AppState;

pub mod input;
pub mod menu;
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
            .add_systems(Update, (draw_menu,).run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (fade_animations,).run_if(in_state(AppState::InGame)),
            )
            .add_observer(on_click)
            .add_observer(on_try_select)
            .add_observer(on_try_menu)
            .add_observer(on_menu_effect)
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

#[derive(Debug, Component)]
struct FadeAnimation {
    timer: Timer,
    direction: f32, // 1.0 = fade in, -1.0 = fade out
}

impl Default for FadeAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            direction: 1.0,
        }
    }
}

fn fade_animations(time: Res<Time>, mut query: Query<(&mut Sprite, &mut FadeAnimation)>) {
    for (mut sprite, mut fade) in &mut query {
        fade.timer.tick(time.delta());

        // Update alpha value
        let current_alpha = sprite.color.alpha();
        let elapsed = time.delta().as_millis() as f32;
        let new_alpha = (current_alpha + (elapsed / 100.0 * fade.direction)).clamp(0.0, 1.0);
        sprite.color.set_alpha(new_alpha);

        // Flip direction when timer finishes
        if fade.timer.finished() {
            fade.direction *= -1.0;
        }
    }
}
