use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use common::{
    game::slice::ClientCity,
    geo::{ImaginaryWorldPoint, WorldPoint},
    network::message::ClientToServerInGameMessage,
    space::window::{Resolution, SetWindow},
    world::{CtxTile, Tile},
};
use derive_more::Constructor;

use crate::{
    assets::tile::{layout, TILE_SIZE},
    ingame::{GameSliceResource, HexTile},
    to_server,
    utils::assets::{GameContext, IntoBundle, IntoEntity, CITY_Z, TILE_Z, UNIT_Z},
};
use crate::{
    core::GameSliceUpdated,
    ingame::{HexCity, HexUnit},
};
use common::game::slice::ClientUnit;
use common::game::slice::GameSlice as BaseGameSlice;
use hexx::{shapes, *};

use super::{
    grid::{GridHex, GridHexResource, GridResource},
    move_::CurrentCenter,
};

// #[cfg(feature = "debug_tiles")]
// use crate::utils::debug::DebugDisplay;

pub fn refresh_grid(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<GridResource>,
    current: Res<CurrentCenter>,
) {
    let window = windows.single();
    let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let (camera, cam_transform) = cameras.single();
    if let Ok(world_point) = camera.viewport_to_world_2d(cam_transform, center) {
        let hex_pos = grid.layout.world_pos_to_hex(world_point);
        let Some(hex_tile_meta) = grid.get(&hex_pos) else {
            return;
        };
        if Some(hex_tile_meta.imaginary) == current.0 {
            return;
        }

        let window_width = window.width() * cam_transform.scale().x;
        let window_height = window.height() * cam_transform.scale().y;
        let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as u64;
        let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as u64;
        let tiles_size = tiles_in_height.max(tiles_in_width);
        let tiles_size = tiles_size * 2;

        // FIXME: called multiple time on same tile
        // FIXME: resolution according to window + zoom + hex size
        let window = SetWindow::from_around(
            &hex_tile_meta.imaginary,
            &Resolution::new(tiles_size, tiles_size),
        );
        to_server!(commands, ClientToServerInGameMessage::SetWindow(window));
    }
}

#[derive(Constructor)]
struct GridUpdater<'a> {
    window: &'a Window,
    transform: &'a GlobalTransform,
    slice: &'a BaseGameSlice,
    assets: &'a AssetServer,
    atlas: &'a mut Assets<TextureAtlasLayout>, // FIXME: try solution without mut
}

// FIXME: feature debug_tiles
// FIXME: no need mu if we solve Assets<TextureAtlasLayout> mut
impl GridUpdater<'_> {
    fn grid(
        &mut self,
        commands: &mut Commands,
        center: &ImaginaryWorldPoint,
    ) -> HashMap<Hex, GridHex> {
        let window_width = self.window.width() * self.transform.scale().x;
        let window_height = self.window.height() * self.transform.scale().y;
        let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as i32;
        let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as i32;
        let tiles_size = tiles_in_height.max(tiles_in_width);
        let tiles_size = tiles_size * 2;
        let layout = layout(center);

        let world = self.slice.world();
        let shape = shapes::parallelogram(
            hex(-(tiles_size / 2), -(tiles_size / 2)),
            hex(tiles_size / 2, tiles_size / 2),
        );
        let mut grid = HashMap::with_capacity(shape.len());

        for hex in shape {
            let imaginary =
                world.imaginary_world_point_for_center_rel((hex.x as isize, hex.y as isize));
            let Some(point) =
                world.try_world_point_for_center_rel((hex.x as isize, hex.y as isize))
            else {
                continue;
            };

            let tile = self.tile(commands, hex, &point, &layout);
            let city = self.city(commands, hex, &point, &layout);
            let units = self.units(commands, hex, &point, &layout);

            grid.insert(hex, GridHex::new(imaginary, point, tile, city, units));
        }

        grid
    }

    fn tile(
        &mut self,
        commands: &mut Commands,
        hex: Hex,
        point: &WorldPoint,
        layout: &HexLayout,
    ) -> GridHexResource<CtxTile<Tile>> {
        // TODO: in self
        let mut ctx = GameContext::new(self.assets, self.atlas, layout);
        let tile = self.slice.world().tile(point).clone();
        let entity = tile.entity(commands, &mut ctx, hex, TILE_Z);

        GridHexResource::new(entity, tile)
    }

    fn city(
        &mut self,
        commands: &mut Commands,
        hex: Hex,
        point: &WorldPoint,
        layout: &HexLayout,
    ) -> Option<GridHexResource<ClientCity>> {
        // TODO: in self
        let mut ctx = GameContext::new(self.assets, self.atlas, layout);
        let city = self.slice.city_at(point).cloned();
        let entity = city
            .clone()
            .map(|city| city.entity(commands, &mut ctx, hex, CITY_Z));
        entity.map(|entity| {
            GridHexResource::new(entity, city.expect("In this city map only if Some"))
        })
    }

    fn units(
        &mut self,
        commands: &mut Commands,
        hex: Hex,
        point: &WorldPoint,
        layout: &HexLayout,
    ) -> Option<GridHexResource<Vec<ClientUnit>>> {
        // TODO: in self
        let mut ctx = GameContext::new(self.assets, self.atlas, layout);
        let units = self
            .slice
            .units_at(point)
            .map(|units| units.into_iter().cloned().collect::<Vec<ClientUnit>>());
        let entity = units
            .clone()
            .map(|units| units.entity(commands, &mut ctx, hex, UNIT_Z));
        entity.map(|entity| {
            GridHexResource::new(entity, units.expect("In this units map only if Some"))
        })
    }

    fn update(&mut self, commands: &mut Commands) {
        let world = self.slice.world();
        let center = world.imaginary_world_point_for_center_rel((0, 0));
        let layout = layout(&center);

        let grid = GridResource::new(self.grid(commands, &center), center, layout);
        commands.insert_resource(grid);
    }
}

// FIXME Optimizations :
// - load more than screen
// - despawn only outdated tiles
// - manage unit & cities like tiles at server side
pub fn react_game_slice_updated(
    _trigger: Trigger<GameSliceUpdated>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<HexCity>>,
    units: Query<Entity, With<HexUnit>>,
    game_slice: Res<GameSliceResource>,
    mut center: ResMut<CurrentCenter>,
    // mut camera_initialized: ResMut<CameraInitializedResource>,
) {
    if let Some(slice) = &game_slice.0 {
        info!("Refresh from game slice");

        // FIXME BS NOW: despawn must be in GridUpdater
        let window = windows.single();
        let (_, transform) = cameras.single();

        // Tiles
        for entity in tiles.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Cities
        for entity in cities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Units
        for entity in units.iter() {
            commands.entity(entity).despawn_recursive();
        }

        GridUpdater::new(window, transform, slice, &asset_server, &mut atlas_layouts)
            .update(&mut commands);

        center.0 = Some(slice.world().imaginary_world_point_for_center_rel((0, 0)));
        // if !camera_initialized.0 && center.0.is_some() {
        //     camera_initialized.0 = true;
        //     commands.trigger(CenterCameraOnGrid)
        // }
    }
}

// fn spawn_game_slice<F, T>(
//     commands: &mut Commands,
//     windows: &Query<&Window, With<PrimaryWindow>>,
//     cameras: &Query<(&Camera, &GlobalTransform)>,
//     atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
//     asset_server: &Res<AssetServer>,
//     game_slice: &BaseGameSlice,
//     current: &mut ResMut<CurrentCenter>,
//     getter: F,
//     z: f32,
//     debug: bool,
// ) -> HexGridResource<T>
// where
//     F: Fn(&WorldPoint) -> T,
//     T: IntoBundle + std::fmt::Debug + Clone + Send + Sync + 'static,
// {
//     let window = windows.single();
//     let (_, cam_transform) = cameras.single();
//     let world = game_slice.world();
//     let center = world.imaginary_world_point_for_center_rel((0, 0));

//     current.0 = Some(center);

//     let window_width = window.width() * cam_transform.scale().x;
//     let window_height = window.height() * cam_transform.scale().y;
//     let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as i32;
//     let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as i32;
//     let tiles_size = tiles_in_height.max(tiles_in_width);
//     let tiles_size = tiles_size * 2;

//     let layout = layout(&center);
//     let world = game_slice.world();

//     let shape = shapes::parallelogram(
//         hex(-(tiles_size / 2), -(tiles_size / 2)),
//         hex(tiles_size / 2, tiles_size / 2),
//     );
//     let mut grid = HashMap::with_capacity(shape.len());
//     for hex in shape {
//         let imaginary_world_point =
//             world.imaginary_world_point_for_center_rel((hex.x as isize, hex.y as isize));
//         let world_point = world.try_world_point_for_center_rel((hex.x as isize, hex.y as isize));
//         let Some(items) = world_point.map(|p| getter(&p)) else {
//             continue;
//         };
//         let Some(entity_) = items.bundle(asset_server, atlas_layouts, &layout, hex, z) else {
//             continue;
//         };

//         #[cfg(feature = "debug_tiles")]
//         let mut entity = commands.spawn(entity_);

//         #[cfg(not(feature = "debug_tiles"))]
//         let entity = commands.spawn(entity_);

//         #[cfg(feature = "debug_tiles")]
//         {
//             let debug_info = (hex, world_point).display();
//             let hex_tile_text = (
//                 Text2d(debug_info),
//                 TextColor(Color::BLACK),
//                 TextFont {
//                     font_size: 12.0,
//                     ..default()
//                 },
//                 Transform::from_xyz(0.0, 0.0, z + 0.1),
//             );
//             entity.with_children(|b| {
//                 b.spawn(hex_tile_text);
//             });
//         }

//         let entity = entity.id();
//         let tile = GridTile::new(entity, imaginary_world_point, world_point, items);
//         grid.insert(hex, tile);
//     }

//     HexGridResource::new(grid, layout)
// }

// #[cfg(feature = "debug_tiles")]
// fn debug_tile(world_point: Option<WorldPoint>, z: f32) {
//     let debug_info = (hex, world_point).display();
//     let hex_tile_text = (
//         Text2d(debug_info),
//         TextColor(Color::BLACK),
//         TextFont {
//             font_size: 12.0,
//             ..default()
//         },
//         Transform::from_xyz(0.0, 0.0, z + 0.1),
//     );
// }

#[cfg(test)]
mod test {
    use common::{
        game::slice::GameSlice as BaseGameSlice,
        geo::{ImaginaryWorldPoint, WorldPoint},
        world::{partial::PartialWorld, CtxTile, TerrainType, Tile},
    };
    use hexx::{hex, shapes, Hex};

    #[test]
    fn test_hex_render() {
        // GIVEN
        let original = WorldPoint::new(10, 10);
        let tiles = vec![
            //
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            //
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            //
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            //
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            //
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
            CtxTile::Visible(Tile::new(TerrainType::Plain)),
        ];
        let world = PartialWorld::new(original.into(), 5, 5, tiles);
        let slice = BaseGameSlice::new(world, vec![], vec![]);
        let world = slice.world();
        let world_ref = world.original();
        let shape: Vec<Hex> = shapes::parallelogram(hex(-2, -2), hex(2, 2)).collect();
        let shape_tuple: Vec<(i32, i32)> = shape.iter().map(|p| (p.x, p.y)).collect();

        // WHEN/THEN
        assert_eq!(world_ref, &ImaginaryWorldPoint::new(10, 10));
        assert_eq!(
            shape_tuple,
            vec![
                (-2, -2),
                (-2, -1),
                (-2, 0),
                (-2, 1),
                (-2, 2),
                (-1, -2),
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (-1, 2),
                (0, -2),
                (0, -1),
                (0, 0),
                (0, 1),
                (0, 2),
                (1, -2),
                (1, -1),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, -2),
                (2, -1),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );

        for (relative, expected) in vec![
            ((-2, -2), (10, 10)),
            ((-2, -1), (10, 11)),
            ((-2, 0), (10, 12)),
            ((-2, 1), (10, 13)),
            ((-2, 2), (10, 14)),
            ((-1, -2), (11, 10)),
            ((-1, -1), (11, 11)),
            ((-1, 0), (11, 12)),
            ((-1, 1), (11, 13)),
            ((-1, 2), (11, 14)),
            ((0, -2), (12, 10)),
            ((0, -1), (12, 11)),
            ((0, 0), (12, 12)),
            ((0, 1), (12, 13)),
            ((0, 2), (12, 14)),
            ((1, -2), (13, 10)),
            ((1, -1), (13, 11)),
            ((1, 0), (13, 12)),
            ((1, 1), (13, 13)),
            ((1, 2), (13, 14)),
            ((2, -2), (14, 10)),
            ((2, -1), (14, 11)),
            ((2, 0), (14, 12)),
            ((2, 1), (14, 13)),
            ((2, 2), (14, 14)),
        ] {
            assert_eq!(
                world.try_world_point_for_center_rel(relative),
                Some(expected.into())
            );
        }

        for (relative, expected) in vec![
            ((-2, -2), TerrainType::Plain),
            ((-2, -1), TerrainType::GrassLand),
            ((-2, 0), TerrainType::Plain),
            ((-2, 1), TerrainType::GrassLand),
            ((-2, 2), TerrainType::Plain),
            ((-1, -2), TerrainType::Plain),
            ((-1, -1), TerrainType::GrassLand),
            ((-1, 0), TerrainType::Plain),
            ((-1, 1), TerrainType::GrassLand),
            ((-1, 2), TerrainType::Plain),
            ((0, -2), TerrainType::Plain),
            ((0, -1), TerrainType::GrassLand),
            ((0, 0), TerrainType::Plain),
            ((0, 1), TerrainType::GrassLand),
            ((0, 2), TerrainType::Plain),
            ((1, -2), TerrainType::Plain),
            ((1, -1), TerrainType::GrassLand),
            ((1, 0), TerrainType::Plain),
            ((1, 1), TerrainType::GrassLand),
            ((1, 2), TerrainType::Plain),
            ((2, -2), TerrainType::Plain),
            ((2, -1), TerrainType::GrassLand),
            ((2, 0), TerrainType::Plain),
            ((2, 1), TerrainType::GrassLand),
            ((2, 2), TerrainType::Plain),
        ] {
            let value: Vec<Option<TerrainType>> = world
                .tile(
                    &world
                        .try_world_point_for_center_rel((relative.0 as isize, relative.1 as isize))
                        .unwrap(),
                )
                .iter()
                .map(|v| match v {
                    CtxTile::Outside => None,
                    CtxTile::Visible(tile) => Some(tile.type_()),
                })
                .collect();
            assert_eq!(value, vec![Some(expected)]);
        }
    }
}
