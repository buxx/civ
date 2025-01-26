pub mod window;
use bevy::prelude::*;
use camera::spawn_camera;
use common::network::message::ServerToClientMessage;
use establishment::react_establishment;
use ingame::react_ingame;
use window::react_game_window_updated;

use crate::network::{EstablishmentMessage, InGameMessage, ServerMessage};

pub mod camera;
pub mod establishment;
pub mod ingame;

pub struct CorePlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_observer(react_server)
            .add_observer(react_establishment)
            .add_observer(react_game_window_updated)
            .add_observer(react_ingame);
    }
}

#[derive(Event)]
pub struct GameWindowUpdated;

#[derive(Event)]
pub struct GameSliceUpdated;

fn react_server(trigger: Trigger<ServerMessage>, mut commands: Commands) {
    match &trigger.event().0 {
        ServerToClientMessage::Establishment(message) => {
            //
            commands.trigger(EstablishmentMessage(message.clone()))
        }
        ServerToClientMessage::InGame(message) => {
            //
            commands.trigger(InGameMessage(message.clone()))
        }
    }
}
