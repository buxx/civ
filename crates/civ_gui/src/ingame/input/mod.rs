use bevy::{prelude::*, window::PrimaryWindow};

use crate::map::grid::GridResource;

use super::{LastKnownCursorPositionResource, TryMenu, TrySelect};

pub mod menu;
pub mod select;

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
