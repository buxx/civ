use bevy::{prelude::*, window::PrimaryWindow};
use common::geo::ImaginaryWorldPoint;

use crate::ingame::LastKnownCursorPositionResource;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentGridCenterResource(pub Option<ImaginaryWorldPoint>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DraggingMap(pub bool);

const KEYBOARD_MAP_OFFSET: f32 = 5.0;
const KEYBOARD_MAP_OFFSET_FAST: f32 = 20.0;

#[inline]
fn map_offset_value(keyboard: &Res<ButtonInput<KeyCode>>) -> f32 {
    if keyboard.pressed(KeyCode::ShiftLeft) | keyboard.pressed(KeyCode::ShiftRight) {
        KEYBOARD_MAP_OFFSET_FAST
    } else {
        KEYBOARD_MAP_OFFSET
    }
}

pub fn handle_map_offset_by_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        let translation = camera.single().translation;
        let offset = map_offset_value(&keyboard);
        camera.single_mut().translation = Vec3::new(translation.x - offset, translation.y, 0.);
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        let translation = camera.single().translation;
        let offset = map_offset_value(&keyboard);
        camera.single_mut().translation = Vec3::new(translation.x, translation.y + offset, 0.);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        let translation = camera.single().translation;
        let offset = map_offset_value(&keyboard);
        camera.single_mut().translation = Vec3::new(translation.x + offset, translation.y, 0.);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        let translation = camera.single().translation;
        let offset = map_offset_value(&keyboard);
        camera.single_mut().translation = Vec3::new(translation.x, translation.y - offset, 0.);
    }
}

// TODO: claim map only when "x" tiles distance from last. To avoid refreshing too much times.
pub fn map_dragging(
    buttons: Res<ButtonInput<MouseButton>>,
    last_position: Res<LastKnownCursorPositionResource>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut dragging: ResMut<DraggingMap>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(current_position) = windows.single().cursor_position() {
            if let Ok(mut camera) = camera_query.get_single_mut() {
                dragging.0 = true;

                let delta_x = (last_position.0.x - current_position.x) * camera.scale.x;
                let delta_y = (last_position.0.y - current_position.y) * camera.scale.y;
                camera.translation.x += delta_x;
                camera.translation.y -= delta_y;
                camera.translation.x = camera.translation.x.round();
                camera.translation.y = camera.translation.y.round();
            }
        }
    }
}

pub fn map_dragging_teardown(
    buttons: Res<ButtonInput<MouseButton>>,
    mut dragging: ResMut<DraggingMap>,
) {
    if buttons.just_released(MouseButton::Left) {
        dragging.0 = false;
    }
}
