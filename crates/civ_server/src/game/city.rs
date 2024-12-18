use bon::Builder;
use common::geo::Geo;
use uuid::Uuid;

use common::geo::GeoContext;

#[derive(Builder, Clone)]
pub struct City {
    id: Uuid,
    name: String,
    geo: GeoContext,
}

impl City {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Geo for City {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}
