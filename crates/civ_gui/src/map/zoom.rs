use bevy::{input::mouse::MouseWheel, prelude::*};

enum ZoomType {
    ZoomIn,
    ZoomOut,
}

impl From<f32> for ZoomType {
    fn from(value: f32) -> Self {
        if value.is_sign_negative() {
            return Self::ZoomOut;
        }

        Self::ZoomIn
    }
}

impl ZoomType {
    fn new_scale(&self, scale: f32) -> f32 {
        match self {
            ZoomType::ZoomIn => match scale {
                4.0 => 2.0,
                _ => 1.0,
            },
            ZoomType::ZoomOut => match scale {
                1.0 => 2.0,
                _ => 4.0,
            },
        }
    }
}

pub fn map_zoom(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut event: MessageReader<MouseWheel>,
) {
    if let Ok(mut camera) = camera.single_mut() {
        for ev in event.read() {
            let mut scale = camera.scale;
            let zoom_type: ZoomType = ev.y.into();

            let new_scale = zoom_type.new_scale(scale.y);
            scale.x = new_scale;
            scale.y = new_scale;

            camera.scale = scale;
        }
    }
}
