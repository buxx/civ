use bon::Builder;
use common::{
    game::{
        slice::{ClientUnit, ClientUnitTasks},
        unit::{UnitTask, UnitType},
    },
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
}

impl Geo for Unit {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

impl Into<ClientUnit> for &Unit {
    fn into(self) -> ClientUnit {
        ClientUnit::builder()
            .id(self.id)
            .type_(self.type_.clone())
            .tasks(self.tasks.clone().into())
            .physics(self.geo.clone())
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
