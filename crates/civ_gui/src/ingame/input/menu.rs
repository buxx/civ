use bevy::prelude::*;

use crate::ingame::{
    menu::{unit::UnitMenu, Menu, MenuResource},
    selected::{Selected, SelectedResource, SelectedUnit},
    TryMenu,
};

// TODO: if try menu on unit or city, change selected before try open menu
pub fn on_try_menu(
    trigger: Trigger<TryMenu>,
    selected: Res<SelectedResource>,
    mut menu: ResMut<MenuResource>,
) {
    if let Some(selected) = selected.0 {
        let _hex = trigger.event().0;

        match selected {
            Selected::Unit(SelectedUnit::One(unit_id)) => {
                menu.0 = Some(Menu::UnitMenu(UnitMenu::new(unit_id)));
                println!("menu for {unit_id}");
            }
            Selected::City(_city_id) => {}
        }
    }
}
