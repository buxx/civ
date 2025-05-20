use bevy::{prelude::*, window::PrimaryWindow};
use common::{
    game::slice::{ClientCity, ClientUnit},
    world::{CtxTile, Tile},
};

use crate::map::move_::DraggingMap;

use super::LastKnownCursorPositionResource;

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
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    // grid: Res<HexGridResource<CtxTile<Tile>>>,
    // cities: Res<HexGridResource<Option<ClientCity>>>,
    // units: Res<HexGridResource<Vec<ClientUnit>>>,
    dragging: Res<DraggingMap>,
) {
    // FIXME: not sure done before dragging teardown
    // if dragging.0 {
    //     return;
    // }

    info!("click");

    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    // if let Some(hex) = window
    //     .cursor_position()
    //     .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    //     .map(|p| grid.layout.world_pos_to_hex(p))
    // {
    //     if let Some(Some(city)) = cities.grid.get(&hex).map(|cities| &cities.item) {
    //         println!("{city:?}");
    //         return;
    //     }
    //     if let Some(units) = units.grid.get(&hex) {
    //         println!("{:?}", units.item.first());
    //     }
    // }
}
