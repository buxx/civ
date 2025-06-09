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
use hexx::{Hex, HexLayout};
// use dyn_clone::DynClone;

use crate::{
    assets::tile::{layout, TILES_ATLAS_PATH},
    ingame::{HexCity, HexTile, HexUnit},
    map::{AtlasIndex, AtlasesResource},
};

pub const TILE_Z: f32 = 0.0;
pub const CITY_Z: f32 = 1.0;
pub const UNIT_Z: f32 = 2.0;

fn terrain_type_index(terrain: &TerrainType) -> AtlasIndex {
    match terrain {
        TerrainType::GrassLand => AtlasIndex(0),
        TerrainType::Plain => AtlasIndex(1),
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
    pub assets: &'a AssetServer,
    pub atlases: &'a AtlasesResource,
    pub frame: &'a GameFrame,
}

impl DrawContext<'_> {
    pub fn with(&self, hex: Hex) -> DrawHexContext {
        DrawHexContext::from_ctx(self, hex)
    }
}

#[derive(Constructor)]
pub struct DrawHexContext<'a> {
    pub ctx: &'a DrawContext<'a>,
    pub hex: Hex,
}

impl<'a> std::ops::Deref for DrawHexContext<'a> {
    type Target = DrawContext<'a>;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'a> DrawHexContext<'a> {
    pub fn from_ctx(ctx: &'a DrawContext<'a>, hex: Hex) -> Self {
        Self { ctx, hex }
    }

    pub fn layout(&self) -> HexLayout {
        layout(&self.slice.center())
    }

    pub fn point(&self) -> Option<WorldPoint> {
        self.slice
            .world()
            .try_world_point_for_center_rel((self.hex.x as isize, self.hex.y as isize))
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
        // FIXME: should not do this once (at startup ?)
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let point = ctx.layout().hex_to_world_pos(ctx.hex);
        let atlas_index = match self {
            CtxTile::Outside => AtlasIndex(4),
            CtxTile::Visible(tile) => terrain_type_index(&tile.type_()),
        };

        HexTileBundle::new(
            HexTile,
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(point.x, point.y, z),
        )
    }

    #[cfg(feature = "debug_tiles")]
    fn debug_bundle(&self, ctx: &DrawHexContext, z: f32) -> Option<Self::DebugBundleType> {
        let point = ctx.layout().hex_to_world_pos(ctx.hex);
        let debug_info = format!("{}.{} ({}.{})", ctx.hex.x, ctx.hex.y, point.x, point.y);
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
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let point = ctx.layout().hex_to_world_pos(ctx.hex);
        let atlas_index = AtlasIndex(5);

        // FIXME: Must be computed from list (first, for example)
        HexUnitBundle::new(
            HexUnit,
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles.clone(),
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
        // FIXME: should not do this once (at startup ?)
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let point = ctx.layout().hex_to_world_pos(ctx.hex);
        let atlas_index = AtlasIndex(6);

        // FIXME: Must be computed from list (first, for example)
        HexUnitBundle::new(
            HexUnit,
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles.clone(),
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
        // TODO: use atlas index dedicated to tasks
        let texture = ctx.assets.load(TILES_ATLAS_PATH);

        let atlas_index = match self.type_() {
            ClientTaskType::Idle => todo!(),
            // TODO: Type specific of this atlas
            ClientTaskType::Settle(_) => AtlasIndex(8),
        };

        ClientTaskBundle::new(
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: ctx.atlases.tiles.clone(),
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
            Text2d(format!("{:.2}%", self.current * 100.)),
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
