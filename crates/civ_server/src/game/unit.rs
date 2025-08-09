use bon::Builder;
use civ_derive::Geo;
#[cfg(test)]
use common::geo::WorldPoint;
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

// Note: fields are pub to permit factori usage in other crates ...
#[derive(Debug, Builder, Clone, Serialize, Deserialize, Geo)]
pub struct Unit {
    pub id: UnitId,
    pub flag: Flag,
    pub type_: UnitType,
    pub task: Option<UnitTaskWrapper>,
    pub geo: GeoContext,
    pub can: Vec<UnitCan>,
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

impl IntoClientModel<ClientUnit> for Unit {
    fn into_client(self, _state: &State) -> ClientUnit {
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

#[cfg(test)]
factori!(Unit, {
    default {
        id = UnitId::default(),
        flag = Flag::Abkhazia,
        type_ = UnitType::Settlers,
        task = None,
        geo = GeoContext::new(WorldPoint::new(0, 0)),
        can = Vec::new(),
    }
});

#[derive(Default)]
pub struct UnitCanBuilder {
    // FIXME BS NOW: context references
}

impl UnitCanBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Vec<UnitCan> {
        // FIXME BS NOW
        vec![UnitCan::Settle]
    }
}
