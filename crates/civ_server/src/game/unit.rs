use common::game::slice::ClientUnit;
use uuid::Uuid;

use crate::task::context::PhysicalContext;

use super::physics::Physics;

#[derive(Clone)]
pub struct Unit {
    id: Uuid,
    physics: PhysicalContext,
}

impl Unit {
    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Physics for Unit {
    fn physics(&self) -> &PhysicalContext {
        &self.physics
    }
}

impl Into<ClientUnit> for &Unit {
    fn into(self) -> ClientUnit {
        ClientUnit::new(self.id, self.physics.clone().into())
    }
}
