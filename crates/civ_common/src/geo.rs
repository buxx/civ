use bon::Builder;
use serde::{Deserialize, Serialize};

// TODO: try Into<Physics> ?
pub trait Geo {
    fn geo(&self) -> &GeoContext;
    fn geo_mut(&mut self) -> &mut GeoContext;
}

#[derive(Builder, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GeoContext {
    x: u64,
    y: u64,
}

impl GeoContext {
    pub fn xy(&self) -> (u64, u64) {
        (self.x, self.y)
    }

    pub fn set_xy(&mut self, value: (u64, u64)) {
        self.x = value.0;
        self.y = value.1;
    }
}
