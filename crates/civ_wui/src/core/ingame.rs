use bevy::prelude::*;

use common::network::message::ServerToClientInGameMessage;

use crate::network::InGameMessage;

pub fn react_ingame(trigger: Trigger<InGameMessage>) {
    match &trigger.event().0 {
        ServerToClientInGameMessage::State(message) => todo!(),
        ServerToClientInGameMessage::Notification(level, _) => todo!(),
    }
}
