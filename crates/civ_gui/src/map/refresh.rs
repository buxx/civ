use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use common::{
    game::slice::ClientCity,
    network::message::ClientToServerInGameMessage,
    space::window::{Resolution, SetWindow},
    world::{CtxTile, Tile},
};
use derive_more::Constructor;

use crate::{
    assets::tile::{absolute_layout, relative_layout, TILE_SIZE},
    core::GameSlicePropagated,
    ingame::{GameFrameResource, GameSliceResource, HexTile},
    map::{grid::Grid, WaitingForGameSlice},
    to_server,
    utils::assets::{DrawContext, DrawHexContext, Spawn, CITY_Z, TILE_Z, UNIT_Z},
};
use crate::{
    core::GameSliceUpdated,
    ingame::{HexCity, HexUnit},
};
use common::game::slice::ClientUnit;
use hexx::{shapes, *};

use super::{
    grid::{GridHex, GridHexResource, GridResource},
    move_::CurrentGridCenterResource,
    AtlasesResource,
};

pub fn refresh_grid(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<GridResource>,
    current: Res<CurrentGridCenterResource>,
    mut waiting: ResMut<WaitingForGameSlice>,
) {
    if waiting.0 {
        return;
    }
    let Some(grid) = &grid.0 else { return };

    let window = windows.single();
    let screen_center_point = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let (camera, cam_transform) = cameras.single();
    if let Ok(screen_center_world2d) =
        camera.viewport_to_world_2d(cam_transform, screen_center_point)
    {
        let screen_center_hex = grid.absolute_layout.world_pos_to_hex(screen_center_world2d);
        let Some(screen_center_hex_meta) = grid.get(&screen_center_hex) else {
            return;
        };

        let Some(current) = current.0 else {
            return;
        };

        let diff_x = (screen_center_hex_meta.imaginary.x - current.x).abs();
        let diff_y = (screen_center_hex_meta.imaginary.y - current.y).abs();
        let window_contains_tiles_x =
            window.width() / (TILE_SIZE.x as f32 / cam_transform.scale().x);
        let window_contains_tiles_y =
            window.height() / (TILE_SIZE.y as f32 / cam_transform.scale().y);

        // FIXME: don't work when zoom out ?
        if (diff_x as f32) > (window_contains_tiles_x / 3.)
            || (diff_y as f32) > (window_contains_tiles_y / 3.)
        {
            let window_width = window.width() * cam_transform.scale().x;
            let window_height = window.height() * cam_transform.scale().y;
            let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as u64;
            let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as u64;
            let tiles_size = tiles_in_height.max(tiles_in_width);
            let tiles_size = tiles_size * 2;

            let window = SetWindow::from_around(
                &screen_center_hex_meta.imaginary,
                &Resolution::new(tiles_size, tiles_size),
            );
            waiting.0 = true;
            error!(
                "DEBUG::refresh::new window: (current center: {:?}) window: {:?}",
                current, window
            );
            to_server!(commands, ClientToServerInGameMessage::SetWindow(window));
        }
    }
}

#[derive(Constructor)]
struct GridUpdater<'a> {
    window: &'a Window,
    transform: &'a GlobalTransform,
    tiles: &'a Query<'a, 'a, Entity, With<HexTile>>,
    cities: &'a Query<'a, 'a, Entity, With<HexCity>>,
    units: &'a Query<'a, 'a, Entity, With<HexUnit>>,
}

impl<'a> GridUpdater<'a> {
    fn grid(&mut self, commands: &mut Commands, ctx: &'a DrawContext<'a>) -> HashMap<Hex, GridHex> {
        let window_width = self.window.width() * self.transform.scale().x;
        let window_height = self.window.height() * self.transform.scale().y;
        let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as i32;
        let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as i32;
        let tiles_size = tiles_in_height.max(tiles_in_width);
        let tiles_size = tiles_size * 2;

        let world = ctx.slice.world();
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

            let ctx = DrawHexContext::from_ctx(ctx, hex);

            let tile = self.tile(commands, &ctx);
            let city = self.city(commands, &ctx);
            let units = self.units(commands, &ctx);

            grid.insert(hex, GridHex::new(imaginary, point, tile, city, units));
        }

        grid
    }

    fn tile(
        &self,
        commands: &mut Commands,
        ctx: &DrawHexContext,
    ) -> GridHexResource<CtxTile<Tile>> {
        let point = ctx.point().expect("Tile called only on world point");
        let tile = ctx.slice.world().tile(&point).clone();
        let entity = tile.spawn(commands, ctx, TILE_Z);

        GridHexResource::new(entity, tile)
    }

    fn city(
        &self,
        commands: &mut Commands,
        ctx: &DrawHexContext,
    ) -> Option<GridHexResource<ClientCity>> {
        let point = ctx.point().expect("City called only on world point");
        let city = ctx.slice.city_at(&point).cloned();
        let entity = city.clone().map(|city| city.spawn(commands, ctx, CITY_Z));
        entity.map(|entity| {
            GridHexResource::new(entity, city.expect("In this city map only if Some"))
        })
    }

    fn units(
        &self,
        commands: &mut Commands,
        ctx: &DrawHexContext,
    ) -> Option<GridHexResource<Vec<ClientUnit>>> {
        let point = ctx.point().expect("Units called only on world point");
        let units = ctx
            .slice
            .units_at(&point)
            .map(|units| units.into_iter().cloned().collect::<Vec<ClientUnit>>());
        let entity = units
            .clone()
            .map(|units| units.spawn(commands, ctx, UNIT_Z));
        entity.map(|entity| {
            GridHexResource::new(entity, units.expect("In this units map only if Some"))
        })
    }

    // TODO: despawn/spawn only really out/in in screen
    fn update(&mut self, commands: &mut Commands, ctx: &'a DrawContext<'a>) {
        let world = ctx.slice.world();
        let center = world.imaginary_world_point_for_center_rel((0, 0));
        let absolute_layout = absolute_layout();
        let relative_layout = relative_layout(&center);

        // Tiles
        for entity in self.tiles.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Cities
        for entity in self.cities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Units
        for entity in self.units.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let grid = GridResource::new(Some(Grid::new(
            self.grid(commands, ctx),
            center,
            relative_layout,
            absolute_layout,
        )));
        commands.insert_resource(grid);
    }
}

// FIXME Optimizations :
// - load more than screen
// - despawn only outdated tiles
// - manage unit & cities like tiles at server side
#[allow(clippy::complexity)]
pub fn react_game_slice_updated(
    _trigger: Trigger<GameSliceUpdated>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    atlases: Res<AtlasesResource>,
    assets: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<HexCity>>,
    units: Query<Entity, With<HexUnit>>,
    slice: Res<GameSliceResource>,
    mut center: ResMut<CurrentGridCenterResource>,
    frame: Res<GameFrameResource>,
    mut waiting: ResMut<WaitingForGameSlice>,
    // mut camera_initialized: ResMut<CameraInitializedResource>,
) {
    waiting.0 = false;

    if let (Some(slice), Some(frame)) = (&slice.0, frame.0) {
        info!("Refresh from game slice: {slice:?}");

        // FIXME BS NOW: despawn must be in GridUpdater
        let window = windows.single();
        let (_, transform) = cameras.single();

        let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
        GridUpdater::new(window, transform, &tiles, &cities, &units).update(&mut commands, &ctx);

        center.0 = Some(slice.center());
        // if !camera_initialized.0 && center.0.is_some() {
        //     camera_initialized.0 = true;
        //     commands.trigger(CenterCameraOnGrid)
        // }

        commands.trigger(GameSlicePropagated);
    }
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
            let value = match world.tile(
                &world
                    .try_world_point_for_center_rel((relative.0 as isize, relative.1 as isize))
                    .unwrap(),
            ) {
                CtxTile::Outside => None,
                CtxTile::Visible(tile) => Some(tile.type_()),
            };
            assert_eq!(value, Some(expected));
        }
    }
}
