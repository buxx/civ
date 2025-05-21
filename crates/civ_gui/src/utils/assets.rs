use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::prelude::*,
    hierarchy::{BuildChildren, ChildBuild},
    sprite::{Sprite, TextureAtlas},
    text::{Text2d, TextColor, TextFont},
    transform::components::Transform,
    utils::default,
};
use common::{
    game::slice::{ClientCity, ClientUnit, GameSlice},
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
    fn bundle(&self, ctx: &GameHexContext, z: f32) -> Self::BundleType;

    #[cfg(feature = "debug_tiles")]
    fn debug_bundle(&self, _ctx: &GameHexContext, _z: f32) -> Option<Self::DebugBundleType> {
        None
    }
}

pub trait Spawn: IntoBundle {
    fn spawn(&self, commands: &mut Commands, ctx: &GameHexContext, z: f32) -> Entity {
        let bundle = self.bundle(ctx, z);

        #[cfg(feature = "debug_tiles")]
        let mut entity = commands.spawn(bundle);

        #[cfg(not(feature = "debug_tiles"))]
        let entity = commands.spawn(bundle);

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
}

#[derive(Constructor)]
pub struct GameContext<'a> {
    pub slice: &'a GameSlice,
    pub assets: &'a AssetServer,
    pub atlases: &'a AtlasesResource,
}

impl GameContext<'_> {
    pub fn with(&self, hex: Hex) -> GameHexContext {
        GameHexContext::from_ctx(self, hex)
    }
}

#[derive(Constructor)]
pub struct GameHexContext<'a> {
    pub ctx: &'a GameContext<'a>,
    pub hex: Hex,
}

impl<'a> std::ops::Deref for GameHexContext<'a> {
    type Target = GameContext<'a>;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'a> GameHexContext<'a> {
    pub fn from_ctx(ctx: &'a GameContext<'a>, hex: Hex) -> Self {
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

    fn bundle(&self, ctx: &GameHexContext, z: f32) -> HexTileBundle {
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
    fn debug_bundle(&self, ctx: &GameHexContext, z: f32) -> Option<Self::DebugBundleType> {
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

    fn bundle(&self, ctx: &GameHexContext, z: f32) -> Self::BundleType {
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
// TODO: Derive
impl Spawn for Vec<ClientUnit> {}

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

    fn bundle(&self, ctx: &GameHexContext, z: f32) -> Self::BundleType {
        // FIXME: should not do this once (at startup ?)
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let point = ctx.layout().hex_to_world_pos(ctx.hex);
        let atlas_index = AtlasIndex(4);

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
