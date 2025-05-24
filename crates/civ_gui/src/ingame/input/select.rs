use bevy::prelude::*;

use crate::{
    ingame::selected::{SelectUpdated, Selected, SelectedResource, SelectedUnit},
    map::grid::GridResource,
};

use super::TrySelect;

pub fn on_try_select(
    trigger: Trigger<TrySelect>,
    mut commands: Commands,
    mut selected: ResMut<SelectedResource>,
    grid: Res<GridResource>,
) {
    let hex = trigger.event().0;
    selected.0 = None;

    if let Some(Some(city)) = grid.get(&hex).map(|hex| &hex.city) {
        selected.0 = Some(Selected::City(*city.id()));
    }

    if let Some(Some(units)) = grid.get(&hex).map(|hex| &hex.units) {
        let unit = units.item.first().expect("Unit vector never Some if empty");
        selected.0 = Some(Selected::Unit(SelectedUnit::One(*unit.id())));
    }

    commands.trigger(SelectUpdated::new(hex, selected.0));
}
