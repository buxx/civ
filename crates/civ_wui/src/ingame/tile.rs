use bevy::prelude::*;
use common::game::slice::GameSlice;
use hexx::{shapes, *};

use crate::{
    assets::tile::{layout, texture_atlas_layout, TILES_ATLAS_PATH},
    ingame::HexTile,
    utils::assets::AsAtlasIndex,
};

#[cfg(feature = "debug_tiles")]
use crate::utils::debug::DebugDisplay;

use super::{CurrentCenter, HexGrid, HexTileMeta};

pub fn spawn_tiles(
    commands: &mut Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    game_slice: &GameSlice,
    current: &mut ResMut<CurrentCenter>,
) {
    let texture = asset_server.load(TILES_ATLAS_PATH);
    let atlas_layout = atlas_layouts.add(texture_atlas_layout());
    let world = game_slice.world();
    let center = world.imaginary_world_point_for_center_rel((0, 0));
    let layout = layout(&center);
    current.0 = Some(center);

    // FIXME size according to window + zoom + hex size
    let entities = shapes::parallelogram(hex(-20, -20), hex(20, 20))
        .map(|hex| {
            let imaginary_world_point =
                world.imaginary_world_point_for_center_rel((hex.x as isize, hex.y as isize));
            let world_point =
                world.try_world_point_for_center_rel((hex.x as isize, hex.y as isize));
            let tile = world_point.and_then(|p| world.get_tile(&p));
            let relative_point = layout.hex_to_world_pos(hex);
            let atlas_index = tile.atlas_index();
            let entity_ = hex_tile_entity(&texture, &atlas_layout, relative_point, &atlas_index);

            let mut entity = commands.spawn(entity_);

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
                HexTileMeta::new(
                    entity,
                    imaginary_world_point,
                    world_point,
                    tile.cloned(),
                    atlas_index,
                ),
            )
        })
        .collect();

    commands.insert_resource(HexGrid::new(entities, layout));
}

fn hex_tile_entity(
    texture: &Handle<Image>,
    atlas_layout: &Handle<TextureAtlasLayout>,
    relative_point: Vec2,
    atlas_index: &super::AtlasIndex,
) -> (HexTile, Sprite, Transform) {
    (
        HexTile,
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                index: **atlas_index,
                layout: atlas_layout.clone(),
            }),
            ..default()
        },
        Transform::from_xyz(relative_point.x, relative_point.y, 0.0),
    )
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
                    .get_tile(&world.try_world_point_for_center_rel(relative).unwrap())
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
