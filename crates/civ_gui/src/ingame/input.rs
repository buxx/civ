use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    ingame::selected::{Selected, SelectedUnit},
    map::{grid::GridResource, move_::DraggingMap, AtlasesResource},
    utils::assets::{GameContext, Spawn, TILE_Z},
};

use super::{
    selected::{Select, SelectedResource},
    GameSliceResource, LastKnownCursorPositionResource,
};

pub fn update_last_known_cursor_position(
    mut last_position: ResMut<LastKnownCursorPositionResource>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = windows.single().cursor_position() {
        last_position.0 = position;
    }
}

pub fn on_click(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<GridResource>,
    mut selected: ResMut<SelectedResource>,
    _dragging: Res<DraggingMap>,
    atlases: Res<AtlasesResource>,
    slice: Res<GameSliceResource>,
    assets: Res<AssetServer>,
    select: Query<Entity, With<Select>>,
) {
    // FIXME: not sure done before dragging teardown
    // if dragging.0 {
    //     return;
    // }
    if let Some(slice) = &slice.0 {
        let window = windows.single();
        let (camera, cam_transform) = cameras.single();
        if let Some(hex) = window
            .cursor_position()
            .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
            .map(|p| grid.layout.world_pos_to_hex(p))
        {
            if let Ok(entity) = select.get_single() {
                commands.entity(entity).despawn();
            }
            selected.0 = Selected::Nothing;

            if let Some(Some(city)) = grid.get(&hex).map(|hex| &hex.city) {
                selected.0 = Selected::City(*city.id());
            }

            if let Some(Some(units)) = grid.get(&hex).map(|hex| &hex.units) {
                let unit = units.item.first().expect("Unit vector never Some if empty");
                selected.0 = Selected::Unit(SelectedUnit::One(*unit.id()));
            }

            match selected.0 {
                Selected::Unit(_) => {
                    let ctx = GameContext::new(slice, &assets, &atlases);
                    let ctx = ctx.with(hex);
                    Select::new(selected.0).spawn(&mut commands, &ctx, TILE_Z + 0.2);
                }
                Selected::Nothing | Selected::City(_) => {}
            }
        }
    }
}
