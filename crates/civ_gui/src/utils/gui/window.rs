use bevy::prelude::*;
use common::space::window::Resolution;

use crate::assets::tile::TILE_SIZE;

pub trait IntoResolution {
    fn resolution(&self) -> Resolution;
}

impl IntoResolution for (&Window, &GlobalTransform) {
    fn resolution(&self) -> Resolution {
        let window = self.0;
        let cam_transform = self.1;

        let window_width = window.width() * cam_transform.scale().x;
        let window_height = window.height() * cam_transform.scale().y;
        let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as u64;
        let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as u64;
        let tiles_size = tiles_in_height.max(tiles_in_width);
        let tiles_size = tiles_size * 2;

        Resolution::new(tiles_size, tiles_size)
    }
}
