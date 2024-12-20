use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct WorldPoint {
    pub x: u64,
    pub y: u64,
}

impl WorldPoint {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

pub trait Geo {
    fn geo(&self) -> &GeoContext;
    fn geo_mut(&mut self) -> &mut GeoContext;
}

#[derive(Builder, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct GeoContext {
    point: WorldPoint,
}

impl GeoContext {
    pub fn point(&self) -> &WorldPoint {
        &self.point
    }

    pub fn set_point(&mut self, value: WorldPoint) {
        self.point = value
    }
}
