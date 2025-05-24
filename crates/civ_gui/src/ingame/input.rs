use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    ingame::selected::{Selected, SelectedUnit},
    map::grid::GridResource,
};

use super::{
    selected::{SelectUpdated, SelectedResource},
    LastKnownCursorPositionResource, TryMenu, TrySelect,
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
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<GridResource>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(hex) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
        .map(|p| grid.layout.world_pos_to_hex(p))
    {
        match click.event().button {
            PointerButton::Primary => commands.trigger(TrySelect(hex)),
            PointerButton::Secondary => commands.trigger(TryMenu(hex)),
            PointerButton::Middle => {}
        };
    }
}

pub fn on_try_select(
    trigger: Trigger<TrySelect>,
    mut commands: Commands,
    mut selected: ResMut<SelectedResource>,
    grid: Res<GridResource>,
) {
    let hex = trigger.event().0;
    selected.0 = Selected::Nothing;

    if let Some(Some(city)) = grid.get(&hex).map(|hex| &hex.city) {
        selected.0 = Selected::City(*city.id());
    }

    if let Some(Some(units)) = grid.get(&hex).map(|hex| &hex.units) {
        let unit = units.item.first().expect("Unit vector never Some if empty");
        selected.0 = Selected::Unit(SelectedUnit::One(*unit.id()));
    }

    commands.trigger(SelectUpdated::new(hex, selected.0));
}

pub fn on_try_menu(_trigger: Trigger<TryMenu>) {
    println!("menu");
}
