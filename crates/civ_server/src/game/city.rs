use bon::Builder;
use common::{game::slice::ClientCity, geo::Geo};
use uuid::Uuid;

use common::geo::GeoContext;

#[derive(Debug, Builder, Clone)]
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

pub trait IntoClientCity {
    fn into_client(&self) -> ClientCity;
}

impl IntoClientCity for City {
    fn into_client(&self) -> ClientCity {
        ClientCity::new(self.id(), self.name().to_string(), self.geo().clone())
    }
}
