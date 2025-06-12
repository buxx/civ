pub mod tile;
use bevy::prelude::*;
use derive_more::Constructor;
use grid::{CurrentCursorHex, GridResource};
use move_::{
    handle_map_offset_by_keys, map_dragging, map_dragging_teardown, react_center_camera_on_grid,
    CurrentGridCenterResource, DraggingMap,
};
use refresh::{react_game_slice_updated, refresh_grid};
use std::ops::Deref;
use zoom::map_zoom;

use crate::{
    assets::tile::tiles_texture_atlas_layout, ingame::input::update_last_known_cursor_position,
    state::AppState,
};

pub mod grid;
pub mod move_;
pub mod refresh;
pub mod zoom;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridResource>()
            .init_resource::<CurrentCursorHex>()
            .init_resource::<CurrentGridCenterResource>()
            .init_resource::<DraggingMap>()
            .init_resource::<WaitingForGameSlice>()
            .add_observer(react_game_slice_updated)
            .add_observer(react_center_camera_on_grid)
            // TODO: move into atlases
            .add_systems(Startup, init_atlases)
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

// FIXME BS NOW: move
#[derive(Resource, Constructor)]
pub struct AtlasesResource {
    pub tiles: Handle<TextureAtlasLayout>,
}

fn init_atlases(mut commands: Commands, mut atlas: ResMut<Assets<TextureAtlasLayout>>) {
    let tiles = atlas.add(tiles_texture_atlas_layout());
    commands.insert_resource(AtlasesResource::new(tiles));
}

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct WaitingForGameSlice(pub bool);
