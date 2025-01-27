use bevy::prelude::*;
use common::game::slice::ClientUnit;

use crate::ingame::Unit;

pub fn unit_bundle(unit: &ClientUnit) -> (Unit, Transform) {
    let translation = Vec3::ZERO; // FIXME: real position
    (
        Unit(unit.clone()),
        Transform {
            translation,
            ..default()
        },
    )
}
