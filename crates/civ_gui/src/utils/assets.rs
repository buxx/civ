use bevy::{
    asset::{AssetServer, Assets},
    ecs::prelude::*,
    sprite::{Sprite, TextureAtlas, TextureAtlasLayout},
    text::{Text2d, TextColor, TextFont},
    transform::components::Transform,
    utils::default,
};
use common::{
    game::slice::{ClientCity, ClientUnit},
    world::{CtxTile, TerrainType, Tile},
};
use derive_more::Constructor;
use hexx::{Hex, HexLayout};
// use dyn_clone::DynClone;

use crate::{
    assets::tile::{tiles_texture_atlas_layout, TILES_ATLAS_PATH},
    ingame::{HexCity, HexTile, HexUnit},
    map::AtlasIndex,
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
    fn bundle(&self, ctx: &mut GameContext, hex: Hex, z: f32) -> Self::BundleType;

    #[cfg(feature = "debug_tiles")]
    fn debug_bundle(&self) -> Option<Self::DebugBundleType> {
        None
    }
}

pub trait IntoEntity: IntoBundle {
    fn entity(&self, commands: &mut Commands, ctx: &mut GameContext, hex: Hex, z: f32) -> Entity {
        let bundle = self.bundle(ctx, hex, z);
        commands.spawn(bundle).id()
    }
}

#[derive(Constructor)]
pub struct GameContext<'a> {
    pub assets: &'a AssetServer,
    pub atlas_layouts: &'a mut Assets<TextureAtlasLayout>,
    pub layout: &'a HexLayout,
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

    fn bundle(&self, ctx: &mut GameContext, hex: Hex, z: f32) -> HexTileBundle {
        // FIXME: should not do this once (at startup ?)
        let atlas_layout = ctx.atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let relative_point = ctx.layout.hex_to_world_pos(hex);
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
                    layout: atlas_layout.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(relative_point.x, relative_point.y, z),
        )
    }
}

// TODO: Derive
impl IntoEntity for CtxTile<Tile> {}

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

    fn bundle(&self, ctx: &mut GameContext, hex: Hex, z: f32) -> Self::BundleType {
        // FIXME: should not do this once (at startup ?)
        let atlas_layout = ctx.atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let relative_point = ctx.layout.hex_to_world_pos(hex);
        let atlas_index = AtlasIndex(5);

        // FIXME: Must be computed from list (first, for example)
        HexUnitBundle::new(
            HexUnit,
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: atlas_layout.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(relative_point.x, relative_point.y, z),
        )
    }
}
// TODO: Derive
impl IntoEntity for Vec<ClientUnit> {}

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

    fn bundle(&self, ctx: &mut GameContext, hex: Hex, z: f32) -> Self::BundleType {
        // FIXME: should not do this once (at startup ?)
        let atlas_layout = ctx.atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let relative_point = ctx.layout.hex_to_world_pos(hex);
        let atlas_index = AtlasIndex(4);

        // FIXME: Must be computed from list (first, for example)
        HexUnitBundle::new(
            HexUnit,
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: *atlas_index,
                    layout: atlas_layout.clone(),
                }),
                ..default()
            },
            Transform::from_xyz(relative_point.x, relative_point.y, z),
        )
    }
}
// TODO: Derive
impl IntoEntity for ClientCity {}
