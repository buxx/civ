use bon::Builder;
use common::{game::slice::ClientCity, geo::Geo};
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
}

impl Geo for City {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

impl Into<ClientCity> for &City {
    fn into(self) -> ClientCity {
        ClientCity::new(self.id, self.name.clone(), self.geo.clone().into())
    }
}
