use bevy::prelude::*;
use common::game::unit::UnitId;

#[derive(Debug, Event, Deref)]
pub struct SetupSettle(pub UnitId);

pub fn on_setup_settle(trigger: Trigger<SetupSettle>) {
    info!("Setup settle {} !", trigger.event().0)
}
