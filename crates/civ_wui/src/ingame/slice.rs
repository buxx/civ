use bevy::prelude::*;

use crate::core::GameSliceUpdated;

pub fn react_game_slice_updated(trigger: Trigger<GameSliceUpdated>) {
    // FIXME: despawn(outdated)/spawn(new) tiles, cities, units
}
