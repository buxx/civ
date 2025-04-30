use bevy::prelude::*;

use crate::{core::GameSliceUpdated, ingame::GameWindow};

pub fn react_game_window_updated(
    _trigger: Trigger<GameSliceUpdated>,
    window: Res<GameWindow>,
    mut _camera: Query<&mut Transform, With<Camera2d>>,
) {
    if let Some(_window) = &window.0 {
        // let offset_x = TILE_SIZE.x * window.start().x as u32;
        // let offset_y = TILE_SIZE.y * window.start().y as u32;
        // camera.single_mut().translation = Vec3::new(offset_x as f32, offset_y as f32, 0.);
        // info!(
        //     "camera translation: {:?} ({:?})",
        //     camera.single().translation,
        //     window
        // );
    }
}
