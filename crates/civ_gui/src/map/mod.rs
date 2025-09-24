pub mod tile;
use bevy::prelude::*;
use derive_more::Constructor;
use grid::{CurrentCursorHex, GridResource};
use move_::{
    handle_map_offset_by_keys, map_dragging, map_dragging_teardown, CurrentGridCenterResource,
    DraggingMap,
};
use refresh::{react_game_slice_updated, refresh_grid};
use std::ops::Deref;
use zoom::map_zoom;

use crate::{
    assets::{
        select::{select_texture_atlas_layout, SELECT_ATLAS_PATH},
        tile::{tiles_texture_atlas_layout, TILES_ATLAS_PATH},
        unit::{units_texture_atlas_layout, UNITS_ATLAS_PATH},
    },
    ingame::input::update_last_known_cursor_position,
    map::refresh::{
        react_city_removed, react_city_updated, react_unit_removed, react_unit_updated,
    },
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
            .add_observer(react_city_updated)
            .add_observer(react_city_removed)
            .add_observer(react_unit_updated)
            .add_observer(react_unit_removed)
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

        #[cfg(feature = "debug_tiles")]
        {
            use crate::map::grid::CurrentHoverDebugTile;

            app.init_resource::<CurrentHoverDebugTile>();
        }
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

// FIXME BS NOW: move
#[derive(Resource, Constructor)]
pub struct AtlasesResource {
    pub tiles_atlas: Handle<TextureAtlasLayout>,
    pub tiles_texture: Handle<Image>,
    pub units_atlas: Handle<TextureAtlasLayout>,
    pub units_texture: Handle<Image>,
    pub select_atlas: Handle<TextureAtlasLayout>,
    pub select_texture: Handle<Image>,
}

fn init_atlases(
    mut commands: Commands,
    mut atlas: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    let tiles_atlas = atlas.add(tiles_texture_atlas_layout());
    let units_atlas = atlas.add(units_texture_atlas_layout());
    let select_atlas = atlas.add(select_texture_atlas_layout());

    let tiles_texture = assets.load(TILES_ATLAS_PATH);
    let units_texture = assets.load(UNITS_ATLAS_PATH);
    let select_texture = assets.load(SELECT_ATLAS_PATH);

    commands.insert_resource(AtlasesResource::new(
        tiles_atlas,
        tiles_texture,
        units_atlas,
        units_texture,
        select_atlas,
        select_texture,
    ));
}

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct WaitingForGameSlice(pub bool);
