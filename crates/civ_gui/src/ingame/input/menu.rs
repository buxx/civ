use bevy::prelude::*;

use crate::{
    ingame::{
        menu::{city::SetupCityMenu, unit::SetupUnitMenu},
        selected::{SelectUpdated, Selected, SelectedResource, SelectedUnit},
        TryMenu,
    },
    map::grid::GridResource,
};

pub fn on_try_menu(
    trigger: Trigger<TryMenu>,
    grid: Res<GridResource>,
    selected: Res<SelectedResource>,
    mut commands: Commands,
) {
    let hex = trigger.event().0;

    if let Some(grid) = &grid.0 {
        if let Some(grid_hex) = grid.get(&hex) {
            // Try menu on city is prior than units or current selection
            if let Some(city) = &grid_hex.city {
                commands.trigger(SelectUpdated::new(hex, Some(Selected::City(*city.id()))));
                commands.trigger(SetupCityMenu(*city.id()));
                return;
            }

            // Try menu on units is prior to currently selection
            if let Some(units) = &grid_hex.units {
                if let Some(unit) = units.first() {
                    commands.trigger(SelectUpdated::new(
                        hex,
                        Some(Selected::Unit(SelectedUnit::One(*unit.id()))),
                    ));
                    commands.trigger(SetupUnitMenu(*unit.id()));
                    return;
                }
            }
        }
    }

    // Is selection, open menu on it
    if let Some(selected) = selected.0 {
        match selected {
            Selected::Unit(SelectedUnit::One(unit_id)) => {
                commands.trigger(SetupUnitMenu(unit_id));
            }
            Selected::City(_city_id) => {}
        }
    }
}
