use bevy::prelude::*;
use common::network::message::{
    ClientToServerEstablishmentMessage, ClientToServerGameMessage, ClientToServerMessage,
};

use crate::{bridge::SendMessageToServerEvent, menu::join::TakePlaceEvent};

pub fn take_place(trigger: Trigger<TakePlaceEvent>, mut commands: Commands) {
    let flag = trigger.event().0;
    info!("Take place as {}", &flag);
    commands.trigger(SendMessageToServerEvent(ClientToServerMessage::Game(
        ClientToServerGameMessage::Establishment(ClientToServerEstablishmentMessage::TakePlace(
            flag,
        )),
    )));
}
