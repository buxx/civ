use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        nation::flag::Flag,
        slice::ClientUnit,
        unit::{UnitCan, UnitId, UnitType},
    },
    geo::Geo,
};

use common::geo::GeoContext;
use serde::{Deserialize, Serialize};

use crate::{state::State, task::unit::UnitTaskWrapper};

use super::IntoClientModel;

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct Unit {
    id: UnitId,
    flag: Flag,
    type_: UnitType,
    task: Option<UnitTaskWrapper>,
    geo: GeoContext,
    can: Vec<UnitCan>,
}

impl Unit {
    pub fn id(&self) -> &UnitId {
        &self.id
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn task(&self) -> &Option<UnitTaskWrapper> {
        &self.task
    }

    pub fn set_task(&mut self, task: Option<UnitTaskWrapper>) {
        self.task = task;
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

impl IntoClientModel<ClientUnit> for Unit {
    fn into_client(self, _state: &RwLockReadGuard<State>) -> ClientUnit {
        ClientUnit::builder()
            .id(self.id)
            .type_(self.type_)
            .maybe_task(self.task.clone().map(|t| t.into()))
            .geo(self.geo)
            .flag(self.flag)
            .can(self.can.clone())
            .build()
    }
}

pub struct UnitCanBuilder {
    // FIXME BS NOW: context references
}

impl UnitCanBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Vec<UnitCan> {
        // FIXME BS NOW
        return vec![UnitCan::Settle];
    }
}
