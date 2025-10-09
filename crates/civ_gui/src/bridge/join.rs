use bevy::prelude::*;
use common::{
    network::{
        message::{ClientToServerMessage, ClientToServerNetworkMessage},
        Client,
    },
    space::window::Resolution,
};

use crate::{bridge::SendMessageToServerEvent, menu::join::JoinEvent, state::ClientIdResource};

pub fn join(trigger: On<JoinEvent>, mut commands: Commands, client_id: Res<ClientIdResource>) {
    let player_id = trigger.event().0;
    let client_id = client_id.0;
    info!(
        "Joining as player {} and client {} ...",
        &player_id, &client_id
    );
    commands.trigger(SendMessageToServerEvent(ClientToServerMessage::Network(
        ClientToServerNetworkMessage::Hello(
            Client::new(client_id, player_id),
            // FIXME BS NOW: now now now
            Resolution::new(10, 10),
        ),
    )));
}
