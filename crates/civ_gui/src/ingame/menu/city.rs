use bevy::prelude::*;
use common::game::city::CityId;

#[derive(Debug, Event)]
pub struct SetupCityMenu(pub CityId);
