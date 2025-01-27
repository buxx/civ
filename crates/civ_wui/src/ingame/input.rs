use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use common::{
    network::message::{
        ClientToServerGameMessage, ClientToServerInGameMessage, ClientToServerMessage,
    },
    space::window::{Resolution, SetWindow},
};

use crate::{assets::tile::layout, network::ClientToServerSenderResource};

use super::{CurrentCenter, CurrentCursorHex, DraggingMap, HexGrid, LastKnownCursorPosition};

const KEYBOARD_MAP_OFFSET: f32 = 5.0;
const KEYBOARD_MAP_OFFSET_FAST: f32 = 20.0;

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

#[inline]
fn map_offset_value(keyboard: &Res<ButtonInput<KeyCode>>) -> f32 {
    if keyboard.pressed(KeyCode::ShiftLeft) | keyboard.pressed(KeyCode::ShiftRight) {
        KEYBOARD_MAP_OFFSET_FAST
    } else {
        KEYBOARD_MAP_OFFSET
    }
}

pub fn update_last_known_cursor_position(
    mut last_position: ResMut<LastKnownCursorPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = windows.single().cursor_position() {
        last_position.0 = position;
    }
}

// TODO: claim map only when "x" tiles distance from last. To avoid refreshing too much times.
pub fn map_dragging(
    buttons: Res<ButtonInput<MouseButton>>,
    last_position: Res<LastKnownCursorPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut dragging: ResMut<DraggingMap>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(current_position) = windows.single().cursor_position() {
            if let Ok(mut camera) = camera_query.get_single_mut() {
                dragging.0 = true;

                let delta_x = last_position.0.x - current_position.x;
                let delta_y = last_position.0.y - current_position.y;
                camera.translation.x += delta_x;
                camera.translation.y -= delta_y;
            }
        }
    }
}

// pub fn map_dragging(
//     buttons: Res<ButtonInput<MouseButton>>,
//     position: Res<LastKnownCursorPosition>,
//     mut camera_query: Query<&mut Transform, With<Camera2d>>,
//     mut evr_motion: EventReader<MouseMotion>,
//     mut dragging: ResMut<DraggingMap>,
//     mut last_motion: Local<Vec2>, // Store unprocessed motion
// ) {
//     if buttons.pressed(MouseButton::Left) {
//         if let Ok(mut camera) = camera_query.get_single_mut() {
//             dragging.0 = true;

//             // Accumulate new motion events
//             for ev in evr_motion.read() {
//                 *last_motion += ev.delta;
//             }

//             // Apply accumulated motion every frame
//             if last_motion.length_squared() > 0.0 {
//                 camera.translation.x -= last_motion.x;
//                 camera.translation.y += last_motion.y;
//                 *last_motion = Vec2::ZERO; // Reset after applying
//             }
//         }
//     } else {
//         *last_motion = Vec2::ZERO; // Reset when not dragging
//     }
// }

pub fn map_zoom(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut scale = camera.single().scale;
                scale.x += ev.y;
                scale.y += ev.y;
                scale.x = scale.x.max(1.0);
                scale.y = scale.y.max(1.0);
                camera.single_mut().scale = scale;
            }
            MouseScrollUnit::Pixel => {
                let mut scale = camera.single().scale;
                scale.x += ev.y / 100.;
                scale.y += ev.y / 100.;
                scale.x = scale.x.max(1.0);
                scale.y = scale.y.max(1.0);
                camera.single_mut().scale = scale;
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

pub fn refresh_tiles(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
    client_to_server: Res<ClientToServerSenderResource>,
    current: Res<CurrentCenter>,
) {
    let window = windows.single();
    let center = Vec2::new(
        window.resolution.width() / 2.0,
        window.resolution.height() / 2.0,
    );
    let (camera, cam_transform) = cameras.single();
    if let Ok(world_point) = camera.viewport_to_world_2d(cam_transform, center) {
        let hex_pos = grid.layout.world_pos_to_hex(world_point);
        let Some(hex_tile_meta) = grid.entities.get(&hex_pos) else {
            return;
        };
        let point = hex_tile_meta.imaginary();
        if Some(point) == current.0 {
            return;
        }

        // FIXME: called multiple time on same tile
        // FIXME: resolution according to window + zoom + hex size
        let set_window = SetWindow::from_around(&point, &Resolution::new(50, 50));
        // TODO: refactor clean
        client_to_server
            .0
            .send_blocking(ClientToServerMessage::Game(
                ClientToServerGameMessage::InGame(ClientToServerInGameMessage::SetWindow(
                    set_window,
                )),
            ))
            .unwrap();
    }
}

pub fn color_tile_on_hover(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut Sprite>,
    mut current_hex: ResMut<CurrentCursorHex>,
    dragging: Res<DraggingMap>,
) {
    if dragging.0 {
        return;
    }

    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(world_point) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let hex_pos = grid.layout.world_pos_to_hex(world_point);
        if Some(hex_pos) == current_hex.0 {
            return;
        }

        {
            let Some(hex_tile_meta) = grid.entities.get(&hex_pos) else {
                return;
            };

            let Ok(mut new_sprite) = tiles.get_mut(hex_tile_meta.entity()) else {
                return;
            };

            if let Some(new_atlas) = new_sprite.texture_atlas.as_mut() {
                new_atlas.index = 2;
            }
        }

        if let Some(current_hex) = current_hex.0 {
            let Some(old_entity) = grid.entities.get(&current_hex) else {
                return;
            };

            let Ok(mut old_sprite) = tiles.get_mut(old_entity.entity()) else {
                return;
            };

            if let Some(old_atlas) = old_sprite.texture_atlas.as_mut() {
                old_atlas.index = **old_entity.atlas();
            }
        }

        current_hex.0 = Some(hex_pos);
    }
}
