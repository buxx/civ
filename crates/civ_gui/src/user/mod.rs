use bevy::prelude::*;
use common::{game::PlayerId, network::ServerAddress};

use crate::core::preferences::PreferencesResource;

pub mod preferences;

pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_player_id)
            .add_observer(update_keep_connected);
    }
}

#[derive(Debug, Event)]
pub struct SetPlayerIdEvent(pub ServerAddress, pub PlayerId);

#[derive(Debug, Event)]
pub struct SetKeepConnectedEvent(pub ServerAddress, pub bool);

fn update_player_id(trigger: On<SetPlayerIdEvent>, mut preferences: ResMut<PreferencesResource>) {
    let event = trigger.event();
    info!("Set player id {} preference for {}", &event.1, &event.0);
    preferences.set_player_id(&event.0, &event.1);
}

fn update_keep_connected(
    trigger: On<SetKeepConnectedEvent>,
    mut preferences: ResMut<PreferencesResource>,
) {
    let event = trigger.event();
    info!(
        "Set keep connected {} preference for {}",
        &event.1, &event.0
    );
    preferences.set_keep_connected(&event.0, event.1);
}
