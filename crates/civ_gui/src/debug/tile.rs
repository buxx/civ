use bevy::{prelude::*, window::PrimaryWindow};
use common::geo::{ImaginaryWorldPoint, WorldPoint};

use crate::{
    assets::tile::TILE_SIZE,
    map::{
        grid::{CurrentCursorHex, CurrentHoverDebugTile, GridResource},
        move_::DraggingMap,
        AtlasIndex,
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
    grid: Res<GridResource>,
    mut tiles: Query<&mut Sprite>,
    mut current_hex: ResMut<CurrentCursorHex>,
    mut current: ResMut<CurrentHoverDebugTile>,
    dragging: Res<DraggingMap>,
) {
    if dragging.0 {
        return;
    }
    let Some(grid) = &grid.0 else { return };

    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(Some(point)) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
        .map(|p| ImaginaryWorldPoint::from_iso(TILE_SIZE, p).into())
    {
        {
            let Some(grid_hex) = grid.get(&point) else {
                return;
            };

            let hover_tile_entity = grid_hex.tile.entity;

            if current.0.as_ref().map(|c| c.0) != Some(hover_tile_entity) {
                // Restore previous (if any) sprite
                if let Some((entity, original_atlas_index)) = &current.0 {
                    let Ok(mut sprite) = tiles.get_mut(*entity) else {
                        return;
                    };

                    if let Some(atlas) = sprite.texture_atlas.as_mut() {
                        atlas.index = original_atlas_index.0;
                    }

                    current.0 = None;
                }

                // Modify newly overed sprite
                let Ok(mut sprite) = tiles.get_mut(grid_hex.tile.entity) else {
                    return;
                };

                if let Some(atlas) = sprite.texture_atlas.as_mut() {
                    current.0 = Some((hover_tile_entity, AtlasIndex(atlas.index)));
                    atlas.index = 100;
                }
            }
        }

        current_hex.0 = Some(point.into());
    }
}
