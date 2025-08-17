use bevy::prelude::*;
use common::geo::GeoContext;

use crate::ingame::{menu::info::SetupTileInfoMenu, GameSliceResource, TryTileInfo};

pub fn on_try_tile_info(
    trigger: Trigger<TryTileInfo>,
    slice: Res<GameSliceResource>,
    mut commands: Commands,
) {
    let hex = trigger.event().0;

    if let Some(slice) = &slice.0 {
        if let Some(point) = slice.try_world_point_for_center_rel((hex.x as isize, hex.y as isize))
        {
            debug!("Open tile info menu for hex {:?} ({:?}", &point, &hex);
            commands.trigger(SetupTileInfoMenu(GeoContext::new(point)));
        }
    }
}
