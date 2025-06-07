use bevy::prelude::*;

use crate::ingame::{
    menu::unit::SetupUnitMenu,
    selected::{Selected, SelectedResource, SelectedUnit},
    GameSliceResource, TryMenu,
};

// TODO: if try menu on unit or city, change selected before try open menu
pub fn on_try_menu(
    trigger: Trigger<TryMenu>,
    selected: Res<SelectedResource>,
    slice: Res<GameSliceResource>,
    mut commands: Commands,
) {
    if let (Some(selected), Some(slice)) = (selected.0, &slice.0) {
        let _hex = trigger.event().0;

        match selected {
            Selected::Unit(SelectedUnit::One(unit_id)) => {
                commands.trigger(SetupUnitMenu::Unit(unit_id));
            }
            Selected::City(_city_id) => {}
        }
    }
}
