use bon::Builder;
use common::game::{
    slice::{ClientUnit, ClientUnitTasks},
    unit::{UnitTask, UnitType},
};
use uuid::Uuid;

use crate::task::context::PhysicalContext;

use super::physics::Physics;

#[derive(Builder, Clone)]
pub struct Unit {
    id: Uuid,
    type_: UnitType,
    #[builder(default)]
    tasks: UnitTasks,
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
        ClientUnit::builder()
            .id(self.id)
            .type_(self.type_.clone())
            .tasks(self.tasks.clone().into())
            .physics(self.physics.clone().into())
            .build()
    }
}

#[derive(Default, Clone)]
pub struct UnitTasks {
    stack: Vec<(Uuid, UnitTask)>,
}

impl Into<ClientUnitTasks> for UnitTasks {
    fn into(self) -> ClientUnitTasks {
        ClientUnitTasks::new(self.stack.clone())
    }
}
