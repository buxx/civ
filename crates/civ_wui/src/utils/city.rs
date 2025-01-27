use bevy::prelude::*;
use common::game::slice::ClientCity;

use crate::ingame::City;

pub fn city_bundle(city: &ClientCity) -> (City, Transform) {
    let translation = Vec3::ZERO; // FIXME: real position
    (
        City(city.clone()),
        Transform {
            translation,
            ..default()
        },
    )
}
