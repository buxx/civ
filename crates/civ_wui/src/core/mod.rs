use bevy::prelude::*;
use camera::spawn_camera;
use common::network::message::ServerToClientMessage;
use ingame::react_server_message;
// use establishment::react_establishment;
use preferences::PreferencesResource;
use window::react_game_window_updated;

use crate::user::preferences::Preferences;

pub mod camera;
pub mod establishment;
pub mod ingame;
pub mod preferences;
pub mod window;

pub struct CorePlugin;

#[allow(unused)]
#[derive(Component)]
pub struct Menu;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let preferences = PreferencesResource::new(Preferences::from_env().unwrap());

        app.insert_resource(preferences)
            .add_systems(Startup, spawn_camera)
            // .add_observer(react_server)
            // .add_observer(react_establishment)
            .add_observer(react_game_window_updated)
            .add_observer(react_server_message);
    }
}

// TODO: move
#[derive(Event)]
pub struct GameWindowUpdated;

// TODO: move
#[derive(Event)]
pub struct GameSliceUpdated;

// // TODO: move in bridge
// fn react_server(trigger: Trigger<ServerMessage>, mut commands: Commands) {
//     match &trigger.event().0 {
//         ServerToClientMessage::Establishment(message) => {
//             //
//             commands.trigger(EstablishmentMessage(message.clone()))
//         }
//         ServerToClientMessage::InGame(message) => {
//             //
//             commands.trigger(InGameMessage(message.clone()))
//         }
//     }
// }
