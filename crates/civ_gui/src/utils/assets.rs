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
    ingame::HexTile,
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
    ) -> Self::BundleType;
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
    ) -> HexTileBundle {
        // FIXME: should not do this once (at startup ?)
        let atlas_layout = atlas_layouts.add(tiles_texture_atlas_layout());
        let texture = assets.load(TILES_ATLAS_PATH);
        let relative_point = layout.hex_to_world_pos(hex);
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

// impl IntoBundle for ClientUnit {
//     fn atlas_index(&self) -> AtlasIndex {
//         AtlasIndex(5)
//     }
// }

// impl IntoBundle for ClientCity {
//     fn atlas_index(&self) -> AtlasIndex {
//         AtlasIndex(4)
//     }
// }
