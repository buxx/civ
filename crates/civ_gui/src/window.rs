use bevy::{prelude::*, window::WindowResolution};

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "civ".to_string(),
            fit_canvas_to_parent: true,
            resolution: WindowResolution::new(640.0, 640.0),
            ..default()
        }),
        ..default()
    }
}
