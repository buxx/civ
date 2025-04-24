use bevy::prelude::*;
use common::{game::PlayerId, network::ServerAddress};

use crate::core::preferences::PreferencesResource;

pub mod preferences;

pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_player_id);
    }
}

#[derive(Debug, Event)]
pub struct SetPlayerIdEvent(pub ServerAddress, pub PlayerId);

fn update_player_id(
    trigger: Trigger<SetPlayerIdEvent>,
    mut preferences: ResMut<PreferencesResource>,
) {
    let event = trigger.event();
    info!("Set player id {} preference for {}", &event.1, &event.0);
    preferences.set_player_id(&event.0, &event.1);
}
