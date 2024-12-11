use bon::Builder;
use uuid::Uuid;

use crate::task::context::PhysicalContext;

use super::Physics;

#[derive(Builder)]
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
}
