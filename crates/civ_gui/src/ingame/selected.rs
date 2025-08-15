use bevy::prelude::*;
use common::game::{city::CityId, unit::UnitId};
use derive_more::Constructor;
use hexx::Hex;

use crate::{
    assets::{atlas, tile::TILES_ATLAS_PATH},
    core::GameSlicePropagated,
    ingame::GameFrameResource,
    map::{AtlasIndex, AtlasesResource},
    utils::assets::{DrawContext, DrawHexContext, IntoBundle, Spawn, TILE_Z},
};

use super::{FadeAnimation, GameSliceResource};

#[derive(Debug, Event, Constructor)]
pub struct SelectUpdated {
    pub hex: Hex,
    pub selected: Option<Selected>,
}

#[derive(Debug, Resource, Default, Deref, Clone)]
pub struct SelectedResource(pub Option<Selected>);

#[derive(Debug, Component, Constructor, Clone, Copy)]
pub struct Select(Selected);

#[derive(Debug, Clone, Copy)]
pub enum Selected {
    City(CityId),
    Unit(SelectedUnit),
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedUnit {
    One(UnitId),
}

impl IntoBundle for Select {
    type BundleType = SelectBundle;
    #[cfg(feature = "debug_tiles")]
    type DebugBundleType = ();

    fn bundle(&self, ctx: &DrawHexContext, z: f32) -> Self::BundleType {
        // FIXME: should not do this once (at startup ?)
        let texture = ctx.assets.load(TILES_ATLAS_PATH);
        let point = ctx.relative_layout().hex_to_world_pos(ctx.hex);
        let atlas_index = atlas::ITEM_SELECTED;

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
            FadeAnimation::default(),
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
    pub fade: FadeAnimation,
}

pub fn on_select_updated(
    trigger: Trigger<SelectUpdated>,
    mut commands: Commands,
    query: Query<Entity, With<Select>>,
    atlases: Res<AtlasesResource>,
    slice: Res<GameSliceResource>,
    assets: Res<AssetServer>,
    frame: Res<GameFrameResource>,
) {
    if let (Some(slice), Some(frame)) = (&slice.0, frame.0) {
        let SelectUpdated { hex, selected } = trigger.event();

        if let Ok(entity) = query.get_single() {
            commands.entity(entity).despawn();
        }

        if let Some(selected) = selected {
            match selected {
                Selected::Unit(_) => {
                    let ctx = DrawContext::new(slice, &assets, &atlases, &frame);
                    let ctx = ctx.with(*hex);
                    Select::new(*selected).spawn(&mut commands, &ctx, TILE_Z + 0.2);
                }
                Selected::City(_) => {}
            }
        }
    }
}

pub fn select_on_game_slice_propagated(
    _trigger: Trigger<GameSlicePropagated>,
    mut commands: Commands,
    slice: Res<GameSliceResource>,
    query: Query<(&Select, Entity)>,
) {
    if let Some(slice) = &slice.0 {
        for (select, entity) in query.iter() {
            match select.0 {
                Selected::Unit(selected_unit) => match selected_unit {
                    SelectedUnit::One(unit_id) => {
                        if !slice.units_map().contains_key(&unit_id) {
                            commands.entity(entity).despawn();
                        }
                    }
                },
                Selected::City(_city_id) => todo!(),
            }
        }
    }
}
