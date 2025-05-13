use bevy::{prelude::*, window::PrimaryWindow};
use common::game::slice::{ClientCity, ClientUnit};

use crate::map::grid::HexGridResource;

use super::LastKnownCursorPositionResource;

pub fn update_last_known_cursor_position(
    mut last_position: ResMut<LastKnownCursorPositionResource>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = windows.single().cursor_position() {
        last_position.0 = position;
    }
}
