use bevy::prelude::*;

use crate::{
    core::GameSliceUpdated,
    utils::{city::city_bundle, unit::unit_bundle},
};

use super::{
    tile::spawn_tiles, CameraInitialized, CenterCameraOnGrid, City, CurrentCenter, GameSlice,
    HexTile, Unit,
};

pub fn react_game_slice_updated(
    _trigger: Trigger<GameSliceUpdated>,
    mut commands: Commands,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<City>>,
    units: Query<Entity, With<Unit>>,
    game_slice: Res<GameSlice>,
    mut center: ResMut<CurrentCenter>,
    mut camera_initialized: ResMut<CameraInitialized>,
) {
    if let Some(game_slice) = &game_slice.0 {
        // Tiles
        for entity in tiles.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_tiles(
            &mut commands,
            atlas_layouts,
            asset_server,
            game_slice,
            &mut center,
        );

        // Cities
        for entity in cities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for city in game_slice.cities() {
            commands.spawn(city_bundle(city));
        }

        // Units
        for entity in units.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for unit in game_slice.units() {
            commands.spawn(unit_bundle(unit));
        }

        if !camera_initialized.0 && center.0.is_some() {
            camera_initialized.0 = true;
            commands.trigger(CenterCameraOnGrid)
        }
    }
}
