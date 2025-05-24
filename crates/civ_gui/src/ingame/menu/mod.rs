pub mod draw;
use bevy::prelude::*;

use unit::UnitMenu;

pub mod unit;

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct MenuResource(pub Option<Menu>);

#[derive(Debug)]
pub enum Menu {
    UnitMenu(UnitMenu),
}
