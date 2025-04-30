use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub fn map_zoom(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut scale = camera.single().scale;
                scale.x += ev.y;
                scale.y += ev.y;
                scale.x = scale.x.max(1.0);
                scale.y = scale.y.max(1.0);
                scale.x = scale.x.round();
                scale.y = scale.y.round();
                camera.single_mut().scale = scale;
            }
            MouseScrollUnit::Pixel => {
                let mut scale = camera.single().scale;
                scale.x += ev.y / 100.;
                scale.y += ev.y / 100.;
                scale.x = scale.x.max(1.0);
                scale.y = scale.y.max(1.0);
                scale.x = scale.x.round();
                scale.y = scale.y.round();
                camera.single_mut().scale = scale;
            }
        }
    }
}
