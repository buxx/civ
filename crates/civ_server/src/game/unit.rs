use bon::Builder;
use common::{
    game::unit::{UnitTask, UnitType},
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

#[derive(Builder, Clone)]
pub struct Unit {
    id: Uuid,
    type_: UnitType,
    #[builder(default)]
    tasks: UnitTasks,
    geo: GeoContext,
}

impl Unit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn tasks(&self) -> &UnitTasks {
        &self.tasks
    }
}

impl Geo for Unit {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

#[derive(Default, Clone)]
pub struct UnitTasks {
    stack: Vec<(Uuid, UnitTask)>,
}

impl UnitTasks {
    pub fn stack(&self) -> &[(Uuid, UnitTask)] {
        &self.stack
    }
}
