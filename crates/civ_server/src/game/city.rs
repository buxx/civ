use bon::Builder;
use common::game::slice::ClientCity;
use uuid::Uuid;

use crate::task::context::PhysicalContext;

use super::physics::Physics;

#[derive(Builder, Clone)]
pub struct City {
    id: Uuid,
    physics: PhysicalContext,
}

impl City {
    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Physics for City {
    fn physics(&self) -> &PhysicalContext {
        &self.physics
    }

    fn physics_mut(&mut self) -> &mut PhysicalContext {
        &mut self.physics
    }
}

impl Into<ClientCity> for &City {
    fn into(self) -> ClientCity {
        ClientCity::new(self.id, self.physics.clone().into())
    }
}
