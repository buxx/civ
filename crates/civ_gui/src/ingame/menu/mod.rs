pub mod draw;
use bevy::prelude::*;

use unit::{UnitMenu, UnitMenuEffect};

pub mod unit;

pub const MENU_DISPLAY_FACTOR: f32 = 1.5;

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct MenuResource(pub Option<Menu>);

#[derive(Debug)]
pub enum Menu {
    UnitMenu(UnitMenu),
}

pub fn on_menu_effect(trigger: Trigger<UnitMenuEffect>) {
    let effect = trigger.event();
    info!("Trigger {:?}", effect);
}
