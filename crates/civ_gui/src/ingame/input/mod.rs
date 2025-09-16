use bevy::{prelude::*, window::PrimaryWindow};
use common::geo::{ImaginaryWorldPoint, WorldPoint};

use crate::{assets::tile::TILE_SIZE, ingame::TryTileInfo};

use super::{LastKnownCursorPositionResource, TryMenu, TrySelect};

pub mod info;
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
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(Some(point)) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
        .map(|p| ImaginaryWorldPoint::from_iso(TILE_SIZE, p).into())
    {
        dbg!(&point);
        match click.event().button {
            PointerButton::Primary => commands.trigger(TrySelect(point)),
            PointerButton::Secondary => commands.trigger(TryMenu(point)),
            PointerButton::Middle => commands.trigger(TryTileInfo(point)),
        };
    }
}
