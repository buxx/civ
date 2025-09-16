use bevy::{prelude::*, window::PrimaryWindow};
use common::{
    game::{city::CityId, slice::ClientCity, unit::UnitId},
    geo::{ImaginaryWorldPoint, WorldPoint},
    network::message::ClientToServerInGameMessage,
    space::window::Resolution,
    world::{CtxTile, Tile},
};
use derive_more::Constructor;
use rustc_hash::FxHashMap;

use crate::{
    assets::tile::TILE_SIZE,
    core::{CityRemoved, CityUpdated, GameSlicePropagated, UnitRemoved, UnitUpdated},
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
    // current: Res<CurrentGridCenterResource>,
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
        let screen_center_world_point =
            ImaginaryWorldPoint::from_iso(TILE_SIZE, screen_center_world2d);
        // dbg!(screen_center_world_point);
        // let screen_center_world_point = ImaginaryWorldPoint::new(
        //     screen_center_world2d.x as i64 / TILE_SIZE.x as i64,
        //     screen_center_world2d.y as i64 / TILE_SIZE.y as i64,
        // );
        let window_contains_tiles_x =
            window.width() / 2.0 / (TILE_SIZE.x as f32 / cam_transform.scale().x);
        let window_contains_tiles_y =
            window.height() / 2.0 / (TILE_SIZE.y as f32 / cam_transform.scale().y);
        let min_diff = window_contains_tiles_x
            .min(window_contains_tiles_y)
            .min(5.0);

        // FIXME BS NOW: real distance from grid.center and screen_center_world_point
        let distance = 0.0;

        if distance as f32 > min_diff {
            let window_width = window.width() * cam_transform.scale().x;
            let window_height = window.height() * cam_transform.scale().y;
            let tiles_in_width = (window_width / (TILE_SIZE.x as f32)) as u64;
            let tiles_in_height = (window_height / (TILE_SIZE.y as f32)) as u64;
            let tiles_size = tiles_in_height.max(tiles_in_width);
            let tiles_size = tiles_size * 2;

            let window = common::space::window::Window::from_around(
                &screen_center_world_point,
                &Resolution::new(tiles_size, tiles_size),
            );
            waiting.0 = true;

            to_server!(commands, ClientToServerInGameMessage::SetWindow(window));
        }
    }
}

#[derive(Constructor)]
struct GridUpdater<'a> {
    window: &'a Window,
    transform: &'a GlobalTransform,
    // TODO: To free-up usage of these objects, set them only for recreate method ?
    tiles: &'a Query<'a, 'a, Entity, With<HexTile>>,
    cities: &'a Query<'a, 'a, Entity, With<HexCity>>,
    units: &'a Query<'a, 'a, Entity, With<HexUnit>>,
}

impl<'a> GridUpdater<'a> {
    fn build_grid(
        &mut self,
        commands: &mut Commands,
        ctx: &'a DrawContext<'a>,
    ) -> FxHashMap<WorldPoint, GridHex> {
        let mut grid = FxHashMap::default();

        for point in ctx.slice.points() {
            if let Some(point) = ctx.slice.world_point(&point) {
                let ctx = DrawHexContext::from_ctx(ctx, point);
                let tile = self.spawn_tile(commands, &ctx);
                let city = self.spawn_city(commands, &ctx);
                let units = self.spawn_units(commands, &ctx);

                grid.insert(point, GridHex::new(point, tile, city, units));
            }
        }

        grid
    }

    fn spawn_tile(
        &self,
        commands: &mut Commands,
        ctx: &DrawHexContext,
    ) -> GridHexResource<CtxTile<Tile>> {
        let point = ctx.point();
        let tile = ctx
            .slice
            .tiles()
            .get(&point)
            .unwrap_or(&CtxTile::Outside)
            .clone();
        let entity = tile.spawn(commands, ctx, TILE_Z);

        GridHexResource::new(entity, tile)
    }

    fn spawn_city(
        &self,
        commands: &mut Commands,
        ctx: &DrawHexContext,
    ) -> Option<GridHexResource<ClientCity>> {
        let point = ctx.point();
        let city = ctx.slice.cities().get(&point).cloned().flatten();
        let entity = city.clone().map(|city| city.spawn(commands, ctx, CITY_Z));
        entity.map(|entity| {
            GridHexResource::new(entity, city.expect("In this city map only if Some"))
        })
    }

    fn spawn_units(
        &self,
        commands: &mut Commands,
        ctx: &DrawHexContext,
    ) -> Option<GridHexResource<Vec<ClientUnit>>> {
        let point = ctx.point();
        let units = ctx.slice.units().get(&point).cloned().flatten();

        #[cfg(feature = "debug")]
        {
            if let Some(units) = &units {
                debug!("Found {} unit to spawn at {:?}", units.len(), ctx.point());
            }
        }

        let entity = units
            .clone()
            .map(|units| units.spawn(commands, ctx, UNIT_Z));
        entity.map(|entity| {
            GridHexResource::new(entity, units.expect("In this units map only if Some"))
        })
    }

    // TODO: despawn/spawn only really out/in in screen
    fn create(&mut self, commands: &mut Commands, ctx: &'a DrawContext<'a>) {
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

        let grid_ = self.build_grid(commands, ctx);
        let center = ctx.slice.center();
        let resource = GridResource::new(Some(Grid::new(grid_, center)));
        commands.insert_resource(resource);
    }

    fn update(
        &self,
        commands: &mut Commands,
        ctx: &'a DrawContext<'a>,
        grid: &mut Grid,
        action: Action,
    ) {
        match &action {
            Action::SetCity(city) => {
                let point = city.geo().point();
                self.despawn_city(grid, point, commands);

                // FIXME BS NOW: When city has been added ?
                if let Some(grid_hex) = grid.get_mut(point) {
                    let ctx = DrawHexContext::from_ctx(ctx, *point);
                    let city = self.spawn_city(commands, &ctx);
                    grid_hex.city = city;
                }
            }
            Action::RemoveCity(_, point) => {
                self.despawn_city(grid, point, commands);
            }
            Action::SetUnit(unit) => {
                let point = unit.geo().point();
                self.despawn_units(grid, point, commands);

                // FIXME BS NOW: When unit has been added ?
                if let Some(grid_hex) = grid.get_mut(point) {
                    let ctx = DrawHexContext::from_ctx(ctx, *point);
                    let units = self.spawn_units(commands, &ctx);
                    grid_hex.units = units;
                }
            }
            Action::RemoveUnit(_, point) => {
                self.despawn_units(grid, point, commands);
                // FIXME BS NOW: respawn (removed unit was maybe not alone)
            }
        }
    }

    fn despawn_city(&self, grid: &mut Grid, point: &WorldPoint, commands: &mut Commands) {
        if let Some(grid) = grid.get(point) {
            if let Some(resource) = &grid.city {
                // TODO: modify GridResource too ? (see refresh.rs, already done here)
                commands.entity(resource.entity).despawn_recursive();
            }
        }
    }

    fn despawn_units(&self, grid: &mut Grid, point: &WorldPoint, commands: &mut Commands) {
        if let Some(grid) = grid.get(point) {
            if let Some(resource) = &grid.units {
                // TODO: modify GridResource too ? (see refresh.rs, already done here)
                commands.entity(resource.entity).despawn_recursive();
            }
        }
    }
}

pub enum Action {
    SetUnit(ClientUnit),
    RemoveUnit(UnitId, WorldPoint),
    SetCity(ClientCity),
    RemoveCity(CityId, WorldPoint),
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
) {
    waiting.0 = false;

    if let (Some(slice), Some(frame)) = (&slice.0, frame.0) {
        info!("Refresh from game slice");
        debug!("Refresh from game slice: {slice:?}");

        // FIXME BS NOW: despawn must be in GridUpdater
        let window = windows.single();
        let (_, transform) = cameras.single();

        let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
        GridUpdater::new(window, transform, &tiles, &cities, &units).create(&mut commands, &ctx);

        center.0 = Some(slice.center());
        commands.trigger(GameSlicePropagated);
    }
}

#[allow(clippy::complexity)]
pub fn react_city_updated(
    trigger: Trigger<CityUpdated>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    slice: Res<GameSliceResource>,
    atlases: Res<AtlasesResource>,
    assets: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<HexCity>>,
    units: Query<Entity, With<HexUnit>>,
    frame: Res<GameFrameResource>,
    mut grid: ResMut<GridResource>,
    mut commands: Commands,
) {
    let city = &trigger.event().0;
    let city_id = city.id();

    if let (Some(slice), Some(frame), Some(grid)) = (&slice.0, frame.0, &mut grid.0) {
        debug!("Set city: {city_id}");

        let window = windows.single();
        let (_, transform) = cameras.single();

        let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
        GridUpdater::new(window, transform, &tiles, &cities, &units).update(
            &mut commands,
            &ctx,
            grid,
            Action::SetCity(city.clone()),
        );

        commands.trigger(GameSlicePropagated);
    }
}

#[allow(clippy::complexity)]
pub fn react_city_removed(
    trigger: Trigger<CityRemoved>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    slice: Res<GameSliceResource>,
    atlases: Res<AtlasesResource>,
    assets: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<HexCity>>,
    units: Query<Entity, With<HexUnit>>,
    frame: Res<GameFrameResource>,
    mut grid: ResMut<GridResource>,
    mut commands: Commands,
) {
    let (city_id, point) = (trigger.event().0, trigger.event().1);

    if let (Some(slice), Some(frame), Some(grid)) = (&slice.0, frame.0, &mut grid.0) {
        debug!("Remove city: {city_id}");

        let window = windows.single();
        let (_, transform) = cameras.single();

        let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
        GridUpdater::new(window, transform, &tiles, &cities, &units).update(
            &mut commands,
            &ctx,
            grid,
            Action::RemoveCity(city_id, point),
        );

        commands.trigger(GameSlicePropagated);
    }
}

#[allow(clippy::complexity)]
pub fn react_unit_updated(
    trigger: Trigger<UnitUpdated>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    slice: Res<GameSliceResource>,
    atlases: Res<AtlasesResource>,
    assets: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<HexCity>>,
    units: Query<Entity, With<HexUnit>>,
    frame: Res<GameFrameResource>,
    mut grid: ResMut<GridResource>,
    mut commands: Commands,
) {
    let unit = &trigger.event().0;
    let unit_id = unit.id();

    if let (Some(slice), Some(frame), Some(grid)) = (&slice.0, frame.0, &mut grid.0) {
        debug!("Set unit: {unit_id}");

        let window = windows.single();
        let (_, transform) = cameras.single();

        let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
        GridUpdater::new(window, transform, &tiles, &cities, &units).update(
            &mut commands,
            &ctx,
            grid,
            Action::SetUnit(unit.clone()),
        );

        commands.trigger(GameSlicePropagated);
    }
}

#[allow(clippy::complexity)]
pub fn react_unit_removed(
    trigger: Trigger<UnitRemoved>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    slice: Res<GameSliceResource>,
    atlases: Res<AtlasesResource>,
    assets: Res<AssetServer>,
    tiles: Query<Entity, With<HexTile>>,
    cities: Query<Entity, With<HexCity>>,
    units: Query<Entity, With<HexUnit>>,
    frame: Res<GameFrameResource>,
    mut grid: ResMut<GridResource>,
    mut commands: Commands,
) {
    let (unit_id, point) = (trigger.event().0, trigger.event().1);

    if let (Some(slice), Some(frame), Some(grid)) = (&slice.0, frame.0, &mut grid.0) {
        debug!("Remove unit: {unit_id}");

        let window = windows.single();
        let (_, transform) = cameras.single();

        let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
        GridUpdater::new(window, transform, &tiles, &cities, &units).update(
            &mut commands,
            &ctx,
            grid,
            Action::RemoveUnit(unit_id, point),
        );

        commands.trigger(GameSlicePropagated);
    }
}

#[cfg(test)]
mod test {
    use common::{
        geo::{ImaginaryWorldPoint, WorldPoint},
        world::{slice::Slice, CtxTile, TerrainType, Tile},
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
        // let world = Slice::new(original.into(), 5, 5, tiles);
        let slice = common::game::slice::GameSlice::new(
            original.into(),
            5,
            5,
            Slice::new(original.into(), 5, 5, tiles),
            Slice::zero(),
            Slice::zero(),
        );
        let tiles = slice.tiles();
        let world_ref = tiles.original();
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
                slice.try_world_point_for_center_rel(relative),
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
            let value = match tiles.get(
                &slice
                    .try_world_point_for_center_rel((relative.0 as isize, relative.1 as isize))
                    .unwrap(),
            ) {
                Some(CtxTile::Outside) | None => None,
                Some(CtxTile::Visible(tile)) => Some(tile.type_()),
            };
            assert_eq!(value, Some(expected));
        }
    }
}
