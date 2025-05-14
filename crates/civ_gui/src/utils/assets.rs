use bevy::{
    asset::{AssetServer, Assets},
    ecs::prelude::*,
    sprite::{Sprite, TextureAtlas, TextureAtlasLayout},
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

fn terrain_type_index(terrain: &TerrainType) -> AtlasIndex {
    match terrain {
        TerrainType::GrassLand => AtlasIndex(0),
        TerrainType::Plain => AtlasIndex(1),
    }
}

pub trait IntoBundle {
    type BundleType: Bundle;

    fn bundle(
        &self,
        assets: &AssetServer,
        atlas_layouts: &mut Assets<TextureAtlasLayout>,
        layout: &HexLayout,
        hex: Hex,
        z: f32,
    ) -> Option<Self::BundleType>;
}
// dyn_clone::clone_trait_object!(IntoBundle);

#[derive(Bundle, Constructor)]
pub struct HexTileBundle {
    pub marker: HexTile,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl IntoBundle for CtxTile<Tile> {
    type BundleType = HexTileBundle;

    fn bundle(
        &self,
        assets: &AssetServer,
        atlas_layouts: &mut Assets<TextureAtlasLayout>,
        layout: &HexLayout,
        hex: Hex,
        z: f32,
    ) -> Option<HexTileBundle> {
        // FIXME: should not do this once (at startup ?)
        let atlas_layout = atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = assets.load(TILES_ATLAS_PATH);
        let relative_point = layout.hex_to_world_pos(hex);
        let atlas_index = match self {
            CtxTile::Outside => AtlasIndex(4),
            CtxTile::Visible(tile) => terrain_type_index(&tile.type_()),
        };

        Some(HexTileBundle::new(
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
        ))
    }
}

#[derive(Bundle, Constructor)]
pub struct HexUnitBundle {
    pub marker: HexUnit,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl IntoBundle for Vec<ClientUnit> {
    type BundleType = HexUnitBundle;

    fn bundle(
        &self,
        assets: &AssetServer,
        atlas_layouts: &mut Assets<TextureAtlasLayout>,
        layout: &HexLayout,
        hex: Hex,
        z: f32,
    ) -> Option<Self::BundleType> {
        if self.is_empty() {
            return None;
        }

        // FIXME: should not do this once (at startup ?)
        let atlas_layout = atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = assets.load(TILES_ATLAS_PATH);
        let relative_point = layout.hex_to_world_pos(hex);
        let atlas_index = AtlasIndex(5);

        // FIXME: Must be computed from list (first, for example)
        Some(HexUnitBundle::new(
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
        ))
    }
}

#[derive(Bundle, Constructor)]
pub struct HexCityBundle {
    pub marker: HexCity,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl IntoBundle for Vec<ClientCity> {
    type BundleType = HexUnitBundle;

    fn bundle(
        &self,
        assets: &AssetServer,
        atlas_layouts: &mut Assets<TextureAtlasLayout>,
        layout: &HexLayout,
        hex: Hex,
        z: f32,
    ) -> Option<Self::BundleType> {
        if self.is_empty() {
            return None;
        }

        // FIXME: should not do this once (at startup ?)
        let atlas_layout = atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = assets.load(TILES_ATLAS_PATH);
        let relative_point = layout.hex_to_world_pos(hex);
        let atlas_index = AtlasIndex(4);

        // FIXME: Must be computed from list (first, for example)
        Some(HexUnitBundle::new(
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
        ))
    }
}
