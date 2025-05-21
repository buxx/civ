use bevy::prelude::*;
use common::game::{city::CityId, unit::UnitId};

#[derive(Debug, Resource, Default, Deref)]
pub struct SelectedResource(pub Selected);

#[derive(Debug)]
pub enum Selected {
    Nothing,
    City(CityId),
    Unit(SelectedUnit),
}

impl Default for Selected {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Debug)]
pub enum SelectedUnit {
    One(UnitId),
    Multiple(Vec<UnitId>),
}
