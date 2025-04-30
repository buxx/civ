use bevy::prelude::*;

use crate::assets::tile::TILE_SIZE;

#[derive(Component)]
pub struct DebugCircle;

pub fn setup_debug_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = [meshes.add(Annulus::new(
        (TILE_SIZE.x as f32 / 2.0) - 5.0,
        TILE_SIZE.x as f32 / 2.0,
    ))];
    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            DebugCircle,
            Mesh2d(shape),
            MeshMaterial2d(materials.add(Color::hsl(360. * i as f32 / 1.0, 0.95, 0.7))),
            Transform::from_xyz(0.0, 0.0, 100.0),
        ));
    }
}

pub fn update_debug_circle_position(
    mut debug_circle: Query<&mut Transform, With<DebugCircle>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    debug_circle.single_mut().translation = cameras.single().1.translation().with_z(100.0);
}
