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
