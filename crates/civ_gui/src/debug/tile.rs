use bevy::{prelude::*, window::PrimaryWindow};
use common::world::{CtxTile, Tile};

use crate::{
    assets::tile::TILE_SIZE,
    map::{
        grid::{CurrentCursorHex, HexGridResource},
        move_::DraggingMap,
    },
};

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

pub fn color_tile_on_hover(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGridResource<CtxTile<Tile>>>,
    mut tiles: Query<&mut Sprite>,
    mut current_hex: ResMut<CurrentCursorHex>,
    dragging: Res<DraggingMap>,
) {
    if dragging.0 {
        return;
    }

    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(world_point) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let hex_pos = grid.layout.world_pos_to_hex(world_point);
        if Some(hex_pos) == current_hex.0 {
            return;
        }

        {
            let Some(hex_tile_meta) = grid.grid.get(&hex_pos) else {
                return;
            };

            let Ok(mut new_sprite) = tiles.get_mut(hex_tile_meta.entity) else {
                return;
            };

            if let Some(new_atlas) = new_sprite.texture_atlas.as_mut() {
                // Find an other way than store atlas index
                // new_atlas.index = 2;
            }
        }

        if let Some(current_hex) = current_hex.0 {
            let Some(old_entity) = grid.grid.get(&current_hex) else {
                return;
            };

            let Ok(mut old_sprite) = tiles.get_mut(old_entity.entity) else {
                return;
            };

            if let Some(old_atlas) = old_sprite.texture_atlas.as_mut() {
                // Find an other way than store atlas index
                // old_atlas.index = *old_entity.atlas;
            }
        }

        current_hex.0 = Some(hex_pos);
    }
}
