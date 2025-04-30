use bevy::{prelude::*, window::PrimaryWindow};

use super::LastKnownCursorPosition;

pub fn update_last_known_cursor_position(
    mut last_position: ResMut<LastKnownCursorPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = windows.single().cursor_position() {
        last_position.0 = position;
    }
}
