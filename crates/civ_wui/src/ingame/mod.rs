use std::ops::Deref;

use bevy::prelude::*;
use bevy::utils::HashMap;
use common::game::slice::ClientCity;
use common::game::{
    slice::ClientUnit, slice::GameSlice as BaseGameSlice, GameFrame as BaseGameFrame,
};
use common::geo::{ImaginaryWorldPoint, WorldPoint};
use common::space::window::Window as BaseWindow;
use common::world::{CtxTile, Tile as BaseTile};
use hexx::{hex, Hex, HexLayout};
use input::{
    color_tile_on_hover, handle_map_offset_by_keys, map_dragging, map_dragging_teardown, map_zoom,
    refresh_tiles, update_last_known_cursor_position,
};
use slice::react_game_slice_updated;

use crate::core::GameSliceUpdated;
use crate::inject::Injection;

pub mod input;
pub mod slice;
pub mod tile;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWindow>()
            .init_resource::<CameraInitialized>()
            .init_resource::<GameFrame>()
            .init_resource::<GameSlice>()
            .init_resource::<HexGrid>()
            .init_resource::<CurrentCursorHex>()
            .init_resource::<CurrentCenter>()
            .init_resource::<DraggingMap>()
            .init_resource::<LastKnownCursorPosition>()
            .add_observer(react_game_slice_updated)
            .add_systems(Startup, inject)
            // TODO: on state ingame only
            .add_systems(
                Update,
                (
                    handle_map_offset_by_keys,
                    map_zoom,
                    map_dragging.before(update_last_known_cursor_position),
                    update_last_known_cursor_position,
                    refresh_tiles,
                    map_dragging_teardown.after(map_dragging),
                    color_tile_on_hover,
                ),
            )
            .add_observer(react_center_camera_on_grid);
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

#[derive(Resource, Default)]
pub struct CameraInitialized(pub bool);

#[derive(Resource, Default)]
pub struct LastKnownCursorPosition(pub Vec2);

#[derive(Event)]
pub struct CenterCameraOnGrid;

#[derive(Resource, Default)]
pub struct GameFrame(pub Option<BaseGameFrame>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameSlice(pub Option<BaseGameSlice>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameWindow(pub Option<BaseWindow>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DraggingMap(pub bool);

#[derive(Component, Debug)]
pub struct HexTile;

#[derive(Component, Deref, DerefMut)]
pub struct Unit(pub ClientUnit);

#[derive(Component, Deref, DerefMut)]
pub struct City(pub ClientCity);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCursorHex(pub Option<Hex>);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCenter(pub Option<ImaginaryWorldPoint>);

#[derive(Component, Deref, DerefMut)]
pub struct Point(pub WorldPoint);

#[derive(Debug)]
pub struct HexTileMeta {
    entity: Entity,
    imaginary: ImaginaryWorldPoint,
    point: Option<WorldPoint>,
    tile: Option<CtxTile<BaseTile>>,
    atlas: AtlasIndex,
}

impl HexTileMeta {
    pub fn new(
        entity: Entity,
        imaginary: ImaginaryWorldPoint,
        point: Option<WorldPoint>,
        tile: Option<CtxTile<BaseTile>>,
        atlas: AtlasIndex,
    ) -> Self {
        Self {
            entity,
            imaginary,
            point,
            tile,
            atlas,
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn tile(&self) -> &Option<CtxTile<BaseTile>> {
        &self.tile
    }

    pub fn atlas(&self) -> &AtlasIndex {
        &self.atlas
    }

    pub fn point(&self) -> Option<WorldPoint> {
        self.point
    }

    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}

#[derive(Debug, Resource, Default)]
pub struct HexGrid {
    // TODO: Vec for perf (with xy position as index)
    pub entities: HashMap<Hex, HexTileMeta>,
    pub layout: HexLayout,
}

impl HexGrid {
    pub fn new(entities: HashMap<Hex, HexTileMeta>, layout: HexLayout) -> Self {
        Self { entities, layout }
    }
}

pub fn inject(
    mut commands: Commands,
    injection: ResMut<Injection>,
    mut game_slice: ResMut<GameSlice>,
) {
    if let Some(game_slice_) = injection.game_slice() {
        game_slice.0 = Some(game_slice_.clone());
        commands.trigger(GameSliceUpdated);
    }
}

fn react_center_camera_on_grid(
    _trigger: Trigger<CenterCameraOnGrid>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    grid: Res<HexGrid>,
) {
    let origin = grid.layout.origin;
    let translation = camera.single().translation;
    let new_translation = Vec3::new(origin.x, origin.y, translation.z);
    camera.single_mut().translation = new_translation;
}
