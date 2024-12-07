use bon::Builder;
use uuid::Uuid;

use crate::task::PhysicalContext;

#[derive(Builder)]
pub struct City {
    id: Uuid,
    physics: PhysicalContext,
}

impl City {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn physics(&self) -> &PhysicalContext {
        &self.physics
    }
}
