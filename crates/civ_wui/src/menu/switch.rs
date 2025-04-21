use bevy::prelude::*;

use super::state::{MenuState, MenuStateResource, Screen};

#[derive(Debug, Event)]
pub enum SwitchMenuScreenEvent {
    Single,
    Join,
}

pub fn switch(trigger: Trigger<SwitchMenuScreenEvent>, mut state: ResMut<MenuStateResource>) {
    match &trigger.event() {
        SwitchMenuScreenEvent::Single => switch_to_single(&mut state.0),
        SwitchMenuScreenEvent::Join => switch_join(&mut state.0),
    }
}

fn switch_to_single(state: &mut MenuState) {
    state.screen = Screen::Single;
}

fn switch_join(state: &mut MenuState) {
    state.screen = Screen::Join;
}
