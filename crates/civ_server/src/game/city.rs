use bon::Builder;
use common::game::slice::ClientCity;
use uuid::Uuid;

use crate::task::context::GeoContext;

use super::physics::Geo;

#[derive(Builder, Clone)]
pub struct City {
    id: Uuid,
    name: String,
    physics: GeoContext,
}

impl City {
    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Geo for City {
    fn physics(&self) -> &GeoContext {
        &self.physics
    }

    fn physics_mut(&mut self) -> &mut GeoContext {
        &mut self.physics
    }
}

impl Into<ClientCity> for &City {
    fn into(self) -> ClientCity {
        ClientCity::new(self.id, self.name.clone(), self.physics.clone().into())
    }
}
