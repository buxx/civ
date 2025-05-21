use bevy::prelude::*;
use common::game::{city::CityId, unit::UnitId};
use derive_more::Constructor;

use crate::{
    assets::tile::TILES_ATLAS_PATH,
    map::AtlasIndex,
    utils::assets::{GameHexContext, IntoBundle, Spawn},
};

#[derive(Debug, Resource, Default, Deref, Clone)]
pub struct SelectedResource(pub Selected);

#[derive(Debug, Component, Constructor, Clone, Copy)]
pub struct Select(Selected);

#[derive(Debug, Clone, Copy)]
pub enum Selected {
    Nothing,
    City(CityId),
    Unit(SelectedUnit),
}

impl Default for Selected {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedUnit {
    One(UnitId),
    // Multiple(Vec<UnitId>),
}

impl IntoBundle for Select {
    type BundleType = SelectBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = ();

    fn bundle(&self, ctx: &GameHexContext, z: f32) -> Self::BundleType {
        // FIXME: should not do this once (at startup ?)
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let point = ctx.layout().hex_to_world_pos(ctx.hex);
        let atlas_index = AtlasIndex(7);

        SelectBundle::new(
            *self,
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

// TODO: derive
impl Spawn for Select {}

#[derive(Debug, Bundle, Constructor)]
pub struct SelectBundle {
    pub marker: Select,
    pub sprite: Sprite,
    pub transform: Transform,
}
