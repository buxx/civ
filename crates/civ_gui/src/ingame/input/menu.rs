use bevy::prelude::*;

use crate::ingame::{
    menu::{unit::UnitMenu, Menu, MenuResource},
    selected::{Selected, SelectedResource, SelectedUnit},
    GameSliceResource, TryMenu,
};

// TODO: if try menu on unit or city, change selected before try open menu
pub fn on_try_menu(
    trigger: Trigger<TryMenu>,
    selected: Res<SelectedResource>,
    slice: Res<GameSliceResource>,
    mut menu: ResMut<MenuResource>,
) {
    if let (Some(selected), Some(slice)) = (selected.0, &slice.0) {
        let _hex = trigger.event().0;

        match selected {
            Selected::Unit(SelectedUnit::One(unit_id)) => {
                let Some(unit) = slice.unit_by_id(&unit_id) else {
                    error!(
                        "Can't build menu for unit {}: not found in game slice.",
                        unit_id
                    );
                    return;
                };
                menu.0 = Some(Menu::UnitMenu(UnitMenu::from_unit(unit)));
                println!("menu for {unit_id}");
            }
            Selected::City(_city_id) => {}
        }
    }
}
