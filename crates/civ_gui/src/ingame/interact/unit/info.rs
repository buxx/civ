use bevy::prelude::*;
use common::game::{tasks::client::ClientTask, unit::UnitId};
use derive_more::Constructor;

use crate::impl_ui_component_resource;

use super::super::UiComponentResource;

#[derive(Debug, Event, Constructor)]
pub struct SetupUnitInfo(pub UnitId, pub Option<ClientTask>);

// FIXME: manage in automatic way invalidation/update of these info (when unit updated)
#[derive(Debug, Constructor)]
pub struct UnitInfo {
    unit_id: UnitId,
    task: Option<ClientTask>,
}

#[derive(Debug, Resource, Default)]
pub struct UnitInfoResource(pub Option<UnitInfo>);
impl_ui_component_resource!(UnitInfoResource, UnitInfo);

pub fn on_setup_unit_info(trigger: Trigger<SetupUnitInfo>, mut modal: ResMut<UnitInfoResource>) {
    let event = trigger.event();
    modal.0 = Some(UnitInfo::new(event.0, event.1.clone()));
}

// FIXME BS NOW: il faut
// Que la construction de UnitInfo soit faite a partir de la lecture de la vrai Unit (dans la slice)
// Et que, lorsque la slice est mise à jour, si cet UiComponent est ouvert, soit automatiquement reconstruit
// avec le nouvel objet ClientUnit ou détruite si il n'existe plus.
// Idem avec unit menu, city name, etc. Il faut que ce soit "lié" automatiquement (macro ui_component ?)
