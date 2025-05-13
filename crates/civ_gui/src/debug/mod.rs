use bevy::prelude::*;

use tile::{color_tile_on_hover, setup_debug_circle, update_debug_circle_position};

pub mod tile;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(Startup, setup_debug_circle)
                .add_systems(Update, (update_debug_circle_position, color_tile_on_hover));
        }
    }
}
