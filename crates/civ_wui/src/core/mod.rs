pub mod establishment;
pub mod ingame;
use bevy::prelude::*;
use common::network::message::ServerToClientMessage;
use establishment::react_establishment;
use ingame::react_ingame;

use crate::network::{EstablishmentMessage, InGameMessage, ServerMessage};

pub struct CorePlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(react_server)
            .add_observer(react_establishment)
            .add_observer(react_ingame);
    }
}

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
