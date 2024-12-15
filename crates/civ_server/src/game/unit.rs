use bon::Builder;
use common::game::{slice::ClientUnit, unit::UnitType};
use uuid::Uuid;

use crate::task::context::PhysicalContext;

use super::physics::Physics;

#[derive(Builder, Clone)]
pub struct Unit {
    id: Uuid,
    type_: UnitType,
    physics: PhysicalContext,
}

impl Unit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }
}

impl Physics for Unit {
    fn physics(&self) -> &PhysicalContext {
        &self.physics
    }

    fn physics_mut(&mut self) -> &mut PhysicalContext {
        &mut self.physics
    }
}

impl Into<ClientUnit> for &Unit {
    fn into(self) -> ClientUnit {
        ClientUnit::new(self.id, self.type_.clone(), self.physics.clone().into())
    }
}
