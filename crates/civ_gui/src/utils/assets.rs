use bevy::{ecs::prelude::*, prelude::*};
use common::{
    game::{
        slice::{ClientCity, ClientUnit, GameSlice},
        tasks::client::{ClientTask, ClientTaskProgress, ClientTaskType},
        GameFrame,
    },
    geo::WorldPoint,
    world::{CtxTile, TerrainType, Tile},
};
use derive_more::Constructor;

use crate::{
    assets::{atlas, tile::TILE_SIZE, unit::UNIT_SIZE},
    ingame::{HexCity, HexTile, HexUnit},
    map::{AtlasIndex, AtlasesResource},
    utils::screen::Isometric,
};

pub const TILE_Z: f32 = 0.0;
pub const CITY_Z: f32 = 1.0;
pub const UNIT_Z: f32 = 2.0;

fn terrain_type_index(terrain: &TerrainType) -> AtlasIndex {
    match terrain {
        TerrainType::GrassLand => atlas::TILE_GRASSLAND,
        TerrainType::Plain => atlas::TILE_PLAIN,
    }
}

pub trait IntoBundle {
    type BundleType: Bundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType: Bundle;

    // FIXME: regroup dependencies in one struct ?
    fn bundle(&self, ctx: &DrawHexContext, z: f32) -> Self::BundleType;

    #[cfg(feature = "debug_tiles")]
    fn debug_bundle(&self, _ctx: &DrawHexContext, _z: f32) -> Option<Self::DebugBundleType> {
        None
    }
}

pub trait Spawn: IntoBundle {
    fn spawn(&self, commands: &mut Commands, ctx: &DrawHexContext, z: f32) -> Entity {
        let bundle = self.bundle(ctx, z);
        let mut entity = commands.spawn(bundle);

        self.spawned(&mut entity, ctx, z);

        #[cfg(feature = "debug_tiles")]
        {
            if let Some(debug) = self.debug_bundle(ctx, z) {
                entity.with_children(|b| {
                    b.spawn(debug);
                });
            }
        }

        entity.id()
    }

    fn spawned(&self, _entity: &mut EntityCommands, _ctx: &DrawHexContext, _z: f32) {}
}

#[derive(Constructor)]
pub struct DrawContext<'a> {
    pub slice: &'a GameSlice,
    pub _assets: &'a AssetServer,
    pub atlases: &'a AtlasesResource,
    pub frame: &'a GameFrame,
}

impl DrawContext<'_> {
    pub fn with(&self, point: WorldPoint) -> DrawHexContext<'_> {
        DrawHexContext::from_ctx(self, point)
    }
}

#[derive(Constructor)]
pub struct DrawHexContext<'a> {
    pub ctx: &'a DrawContext<'a>,
    pub point: WorldPoint,
}

impl<'a> std::ops::Deref for DrawHexContext<'a> {
    type Target = DrawContext<'a>;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'a> DrawHexContext<'a> {
    pub fn from_ctx(ctx: &'a DrawContext<'a>, point: WorldPoint) -> Self {
        Self { ctx, point }
    }

    pub fn point(&self) -> &WorldPoint {
        &self.point
    }
}

// dyn_clone::clone_trait_object!(IntoBundle);

#[derive(Bundle, Constructor)]
pub struct HexTileBundle {
    pub marker: HexTile,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[derive(Bundle, Constructor)]
pub struct DebugHexTileBundle {
    text: Text2d,
    color: TextColor,
    font: TextFont,
    transform: Transform,
}

impl IntoBundle for CtxTile<Tile> {
    type BundleType = HexTileBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = DebugHexTileBundle;

    fn bundle(&self, ctx: &DrawHexContext, z: f32) -> HexTileBundle {
        let point = ctx.point().iso(TILE_SIZE);
        let atlas_index = match self {
            CtxTile::Outside => atlas::TILE_BLACK,
            CtxTile::Visible(tile) => terrain_type_index(&tile.type_()),
        };

        HexTileBundle::new(
            HexTile,
            Sprite {
                image: ctx.atlases.tiles_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles_atlas.clone(),
                }),
                // anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(point.x, point.y, z),
        )
    }

    #[cfg(feature = "debug_tiles")]
    fn debug_bundle(&self, ctx: &DrawHexContext, z: f32) -> Option<Self::DebugBundleType> {
        let point = ctx.point();
        let debug_info = format!("{}.{}", point.x, point.y,);
        Some(DebugHexTileBundle::new(
            Text2d(debug_info),
            TextColor(Color::BLACK),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, z + 0.1),
        ))
    }
}

// TODO: Derive
impl Spawn for CtxTile<Tile> {}

#[derive(Bundle, Constructor)]
pub struct HexUnitBundle {
    pub marker: HexUnit,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl IntoBundle for Vec<ClientUnit> {
    type BundleType = HexUnitBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = ();

    fn bundle(&self, ctx: &DrawHexContext, z: f32) -> Self::BundleType {
        let point = ctx.point().iso(UNIT_SIZE);
        let atlas_index = atlas::UNIT_SETTLER;

        // FIXME: Must be computed from list (first, for example)
        HexUnitBundle::new(
            HexUnit,
            Sprite {
                image: ctx.atlases.units_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.units_atlas.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(point.x, point.y, z),
        )
    }
}

impl Spawn for Vec<ClientUnit> {
    fn spawned(&self, entity: &mut EntityCommands, ctx: &DrawHexContext, z: f32) {
        // TODO: Consider first unit for now ...
        if let Some(unit) = self.first() {
            if let Some(task) = unit.task() {
                entity.with_children(|b| {
                    b.spawn(task.bundle(ctx, z + 0.1));
                    b.spawn(task.progress(ctx.frame).bundle(ctx, z + 0.1));
                });
            }
        }
    }
}

#[derive(Bundle, Constructor)]
pub struct HexCityBundle {
    pub marker: HexCity,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl IntoBundle for ClientCity {
    type BundleType = HexUnitBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = ();

    fn bundle(&self, ctx: &DrawHexContext, z: f32) -> Self::BundleType {
        let point = ctx.point().iso(TILE_SIZE);
        let atlas_index = atlas::CITY_JUNGLE;

        // FIXME: Must be computed from list (first, for example)
        HexUnitBundle::new(
            HexUnit,
            Sprite {
                image: ctx.atlases.tiles_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles_atlas.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(point.x, point.y, z),
        )
    }
}
// TODO: Derive
impl Spawn for ClientCity {}

#[derive(Bundle, Constructor)]
pub struct ClientTaskBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl IntoBundle for ClientTask {
    type BundleType = ClientTaskBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = ();

    fn bundle(&self, ctx: &DrawHexContext, z: f32) -> Self::BundleType {
        let atlas_index = match self.type_() {
            ClientTaskType::Idle => todo!(),
            ClientTaskType::Settle(_) => atlas::ACTION_SETTLE,
        };

        ClientTaskBundle::new(
            Sprite {
                image: ctx.atlases.tiles_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles_atlas.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(0., 0., z),
        )
    }
}

#[derive(Bundle, Constructor)]
pub struct ClientTaskProgressBundle {
    pub text: Text2d,
    pub color: TextColor,
    pub font: TextFont,
    pub transform: Transform,
    pub progress: Progress,
}

// TODO: Move it in more generic mod
#[derive(Debug, Component, Deref, DerefMut)]
pub struct Progress(pub ClientTaskProgress);

impl IntoBundle for ClientTaskProgress {
    type BundleType = ClientTaskProgressBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = ();

    fn bundle(&self, _ctx: &DrawHexContext, z: f32) -> Self::BundleType {
        ClientTaskProgressBundle::new(
            Text2d(format!("{:.0}%", self.current * 100.)),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            Transform::from_xyz(25.0, -25.0, z + 0.1),
            Progress(self.clone()),
        )
    }
}
