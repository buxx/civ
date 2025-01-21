use bevy::{prelude::*, window::WindowResolution};

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "civ".to_string(),
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }
}
