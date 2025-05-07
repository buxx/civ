use bevy::{prelude::*, window::PrimaryWindow};
use common::{
    geo::WorldPoint,
    network::message::ClientToServerInGameMessage,
    space::window::{Resolution, SetWindow},
    world::{CtxTile, Tile},
};

use crate::{
    assets::tile::{layout, texture_atlas_layout, TILES_ATLAS_PATH, TILE_SIZE},
    ingame::{GameSliceResource, HexTile},
    to_server,
    utils::assets::Displayable,
};
use crate::{
    core::GameSliceUpdated,
    ingame::{CameraInitializedResource, City, Unit},
};
use common::game::slice::ClientCity;
use common::game::slice::ClientUnit;
use common::game::slice::GameSlice as BaseGameSlice;
use hexx::{shapes, *};

use super::{grid::HexGridResource, move_::CurrentCenter, tile::HexMeta, CenterCameraOnGrid};

#[cfg(feature = "debug_tiles")]
use crate::utils::debug::DebugDisplay;

pub fn refresh_tiles(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGridResource<CtxTile<Tile>>>,
    current: Res<CurrentCenter>,
) {
    let window = windows.single();
    let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
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

        let window_width = window.width() * cam_transform.scale().x;
        let window_height = window.height() * cam_transform.scale().y;
        let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as u64;
        let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as u64;
        let tiles_size = tiles_in_height.max(tiles_in_width);
        let tiles_size = tiles_size * 2;

        // FIXME: called multiple time on same tile
        // FIXME: resolution according to window + zoom + hex size
        let window = SetWindow::from_around(&point, &Resolution::new(tiles_size, tiles_size));
        to_server!(commands, ClientToServerInGameMessage::SetWindow(window));
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
    cities: Query<Entity, With<City>>,
    units: Query<Entity, With<Unit>>,
    game_slice: Res<GameSliceResource>,
    mut center: ResMut<CurrentCenter>,
    mut camera_initialized: ResMut<CameraInitializedResource>,
) {
    if let Some(game_slice) = &game_slice.0 {
        // Tiles
        for entity in tiles.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_game_slice(
            &mut commands,
            &windows,
            &cameras,
            &mut atlas_layouts,
            &asset_server,
            game_slice,
            &mut center,
            |p| {
                game_slice
                    .world()
                    .tile_at(p)
                    .into_iter()
                    .cloned()
                    .collect::<Vec<CtxTile<Tile>>>()
            },
            HexTile,
        );

        // Cities
        for entity in cities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_game_slice(
            &mut commands,
            &windows,
            &cameras,
            &mut atlas_layouts,
            &asset_server,
            game_slice,
            &mut center,
            |p| {
                game_slice
                    .cities_at(p)
                    .into_iter()
                    .cloned()
                    .collect::<Vec<ClientCity>>()
            },
            City,
        );

        // Units
        for entity in units.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_game_slice(
            &mut commands,
            &windows,
            &cameras,
            &mut atlas_layouts,
            &asset_server,
            game_slice,
            &mut center,
            |p| {
                game_slice
                    .units_at(p)
                    .into_iter()
                    .cloned()
                    .collect::<Vec<ClientUnit>>()
            },
            Unit,
        );

        if !camera_initialized.0 && center.0.is_some() {
            camera_initialized.0 = true;
            commands.trigger(CenterCameraOnGrid)
        }
    }
}

fn spawn_game_slice<F, T, C>(
    commands: &mut Commands,
    windows: &Query<&Window, With<PrimaryWindow>>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    game_slice: &BaseGameSlice,
    current: &mut ResMut<CurrentCenter>,
    getter: F,
    component: C,
) where
    F: Fn(&WorldPoint) -> Vec<T>,
    T: Displayable + Clone + Send + Sync + 'static,
    C: Component + Clone + Copy,
{
    let window = windows.single();
    let (_, cam_transform) = cameras.single();
    let texture = asset_server.load(TILES_ATLAS_PATH);
    // FIXME: .add ?
    let atlas_layout = atlas_layouts.add(texture_atlas_layout());
    let world = game_slice.world();
    let center = world.imaginary_world_point_for_center_rel((0, 0));

    current.0 = Some(center);

    let window_width = window.width() * cam_transform.scale().x;
    let window_height = window.height() * cam_transform.scale().y;
    let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as i32;
    let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as i32;
    let tiles_size = tiles_in_height.max(tiles_in_width);
    let tiles_size = tiles_size * 2;

    let layout = layout(&center);
    let world = game_slice.world();

    let entities = shapes::parallelogram(
        hex(-(tiles_size / 2), -(tiles_size / 2)),
        hex(tiles_size / 2, tiles_size / 2),
    )
    .flat_map(|hex| {
        let imaginary_world_point =
            world.imaginary_world_point_for_center_rel((hex.x as isize, hex.y as isize));
        let world_point = world.try_world_point_for_center_rel((hex.x as isize, hex.y as isize));

        if let Some(items) = world_point.map(|p| getter(&p)) {
            let relative_point = layout.hex_to_world_pos(hex);
            items
                .into_iter()
                .map(|item| {
                    let atlas_index = item.atlas_index();
                    let entity_ = (
                        component,
                        Sprite {
                            image: texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                index: *atlas_index,
                                layout: atlas_layout.clone(),
                            }),
                            ..default()
                        },
                        Transform::from_xyz(relative_point.x, relative_point.y, 0.0),
                    );

                    #[cfg(feature = "debug_tiles")]
                    let mut entity = commands.spawn(entity_);

                    #[cfg(not(feature = "debug_tiles"))]
                    let entity = commands.spawn(entity_);

                    #[cfg(feature = "debug_tiles")]
                    {
                        let debug_info = (hex, world_point).display();
                        let hex_tile_text = (
                            Text2d(debug_info),
                            TextColor(Color::BLACK),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            Transform::from_xyz(0.0, 0.0, 10.0),
                        );
                        entity.with_children(|b| {
                            b.spawn(hex_tile_text);
                        });
                    }
                    let entity = entity.id();

                    (
                        hex,
                        HexMeta::new(
                            entity,
                            imaginary_world_point,
                            world_point,
                            item,
                            atlas_index,
                        ),
                    )
                })
                .collect()
        } else {
            vec![]
        }
    })
    .collect();

    commands.insert_resource(HexGridResource::new(entities, layout));
}

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
            ((-2, -2), Some(TerrainType::Plain)),
            ((-2, -1), Some(TerrainType::GrassLand)),
            ((-2, 0), Some(TerrainType::Plain)),
            ((-2, 1), Some(TerrainType::GrassLand)),
            ((-2, 2), Some(TerrainType::Plain)),
            ((-1, -2), Some(TerrainType::Plain)),
            ((-1, -1), Some(TerrainType::GrassLand)),
            ((-1, 0), Some(TerrainType::Plain)),
            ((-1, 1), Some(TerrainType::GrassLand)),
            ((-1, 2), Some(TerrainType::Plain)),
            ((0, -2), Some(TerrainType::Plain)),
            ((0, -1), Some(TerrainType::GrassLand)),
            ((0, 0), Some(TerrainType::Plain)),
            ((0, 1), Some(TerrainType::GrassLand)),
            ((0, 2), Some(TerrainType::Plain)),
            ((1, -2), Some(TerrainType::Plain)),
            ((1, -1), Some(TerrainType::GrassLand)),
            ((1, 0), Some(TerrainType::Plain)),
            ((1, 1), Some(TerrainType::GrassLand)),
            ((1, 2), Some(TerrainType::Plain)),
            ((2, -2), Some(TerrainType::Plain)),
            ((2, -1), Some(TerrainType::GrassLand)),
            ((2, 0), Some(TerrainType::Plain)),
            ((2, 1), Some(TerrainType::GrassLand)),
            ((2, 2), Some(TerrainType::Plain)),
        ] {
            assert_eq!(
                match world
                    .tile_at(world.try_world_point_for_center_rel(&relative).unwrap())
                    .unwrap()
                {
                    CtxTile::Outside => None,
                    CtxTile::Visible(tile) => Some(tile.type_()),
                },
                expected
            );
        }
    }
}
