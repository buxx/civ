use std::str::FromStr;

use bevy::prelude::*;
use common::{
    game::PlayerId,
    network::message::{
        ClientToServerEstablishmentMessage, ClientToServerGameMessage, ClientToServerMessage,
        ClientToServerNetworkMessage, ServerToClientEstablishmentMessage,
    },
};

use crate::{
    network::{ClientToServerSenderResource, EstablishmentMessage},
    state::{AppState, Client},
};

use super::{
    gui::{FlagInput, PlayerIdInput},
    Connect, Connecting, TakePlace, TakingPlace,
};

pub fn react_connect(
    _trigger: Trigger<Connect>,
    to_server_sender: Res<ClientToServerSenderResource>,
    player_id_input: Res<PlayerIdInput>,
    mut client: ResMut<Client>,
    mut connecting: ResMut<Connecting>,
) {
    if player_id_input.0.is_empty() {
        return;
    }

    client.set_player_id(PlayerId::from_str(&player_id_input.0).unwrap());
    connecting.0 = true;
    to_server_sender
        .0
        .send_blocking(ClientToServerMessage::Network(
            ClientToServerNetworkMessage::Hello(client.clone()),
        ))
        .unwrap();
}

pub fn react_take_place(
    _trigger: Trigger<TakePlace>,
    to_server_sender: Res<ClientToServerSenderResource>,
    flag_input: Res<FlagInput>,
    mut taking_place: ResMut<TakingPlace>,
) {
    if let Some(flag) = flag_input.0 {
        taking_place.0 = true;
        to_server_sender
            .0
            .send_blocking(ClientToServerMessage::Game(
                ClientToServerGameMessage::Establishment(
                    ClientToServerEstablishmentMessage::TakePlace(flag),
                ),
            ))
            .unwrap();
    }
}

pub fn react_establishment(
    trigger: Trigger<EstablishmentMessage>,
    mut next_state: ResMut<NextState<AppState>>,
    mut connecting: ResMut<Connecting>,
    mut taking_place: ResMut<TakingPlace>,
) {
    match &trigger.event().0 {
        ServerToClientEstablishmentMessage::ServerResume(_, flag) => {
            connecting.0 = false;
            taking_place.0 = false;

            if flag.is_some() {
                next_state.set(AppState::InGame);
            }
        }
        ServerToClientEstablishmentMessage::TakePlaceRefused(_reason) => {
            // FIXME: error message display
        }
    }
}
