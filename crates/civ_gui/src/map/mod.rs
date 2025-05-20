pub mod tile;
use bevy::prelude::*;
use grid::{CurrentCursorHex, GridResource};
use move_::{
    handle_map_offset_by_keys, map_dragging, map_dragging_teardown, react_center_camera_on_grid,
    CurrentCenter, DraggingMap,
};
use refresh::{react_game_slice_updated, refresh_grid};
use std::ops::Deref;
use zoom::map_zoom;

use crate::{ingame::input::update_last_known_cursor_position, state::AppState};

pub mod grid;
pub mod move_;
pub mod refresh;
pub mod zoom;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridResource>()
            .init_resource::<CurrentCursorHex>()
            .init_resource::<CurrentCenter>()
            .init_resource::<DraggingMap>()
            .add_observer(react_game_slice_updated)
            .add_observer(react_center_camera_on_grid)
            .add_systems(
                Update,
                (
                    handle_map_offset_by_keys,
                    map_zoom,
                    map_dragging.before(update_last_known_cursor_position),
                    map_dragging_teardown.after(map_dragging),
                    refresh_grid,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Debug)]
pub struct AtlasIndex(pub usize);

impl Deref for AtlasIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Event)]
pub struct CenterCameraOnGrid;
