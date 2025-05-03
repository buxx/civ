use std::collections::HashMap;

use common::{
    game::{nation::flag::Flag, PlayerId},
    geo::GeoContext,
    network::{Client, ClientId},
    space::window::{Resolution, Window},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::effect::ClientEffect;

// FIXME: contain client and player related. rename ? split ?
#[derive(Default)]
pub struct Clients {
    count: usize,
    // FIXME BS NOW: il faut que la window soit cote PlayerId pour être restitue a la connexion
    // FIXME BS NOW: move WindowResolution elsewhere (not stored) ? remove Window
    clients: HashMap<ClientId, (Resolution, Window)>,
    states: HashMap<PlayerId, ClientState>,
}

#[derive(Debug, Error)]
pub enum ClientsError {
    #[error("Unknown client {0}")]
    UnknownClient(ClientId),
    #[error("Unknown player {0}")]
    UnknownPlayer(PlayerId),
}

impl Clients {
    pub fn new(
        clients: HashMap<ClientId, (Resolution, Window)>,
        states: HashMap<PlayerId, ClientState>,
    ) -> Self {
        Self {
            count: 0,
            clients,
            states,
        }
    }

    pub fn with_count(mut self, value: usize) -> Self {
        self.count = value;
        self
    }

    pub fn with_clients(mut self, value: HashMap<ClientId, (Resolution, Window)>) -> Self {
        self.clients = value;
        self
    }

    pub fn with_states(mut self, value: HashMap<PlayerId, ClientState>) -> Self {
        self.states = value;
        self
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn set_count(&mut self, value: usize) {
        self.count = value;
    }

    pub fn apply(&mut self, client: &Client, effect: &ClientEffect) -> Result<(), ClientsError> {
        match effect {
            ClientEffect::PlayerTookPlace(flag, window) => {
                self.states
                    .insert(*client.player_id(), ClientState::new(*flag, window.clone()));
            }
            ClientEffect::SetWindow(window) => {
                if let Some(state) = self.states.get_mut(client.player_id()) {
                    state.set_window(window.clone());
                };
            }
            ClientEffect::SetResolution(resolution) => {
                if let Some((resolution_, _)) = self.clients.get_mut(client.client_id()) {
                    *resolution_ = resolution.clone();
                } else {
                    self.clients
                        .insert(*client.client_id(), (resolution.clone(), Window::default()));
                };
            }
        };

        Ok(())
    }

    pub fn concerned(&self, geo: &GeoContext) -> Vec<ClientId> {
        self.clients
            .iter()
            .filter_map(|(client, (_, window))| {
                if window.contains(geo) {
                    Some(*client)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn client_ids(&self) -> Vec<ClientId> {
        self.clients.keys().copied().collect()
    }

    pub fn player_state(&self, player_id: &PlayerId) -> Option<&ClientState> {
        self.states.get(player_id)
    }

    pub fn flags(&self) -> Vec<Flag> {
        self.states.values().map(|s| *s.flag()).collect()
    }

    pub fn states(&self) -> &HashMap<PlayerId, ClientState> {
        &self.states
    }

    pub fn client_windows(&self) -> &HashMap<ClientId, (Resolution, Window)> {
        &self.clients
    }

    pub fn refresh_count(&mut self) {
        self.count = self.clients.len();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientState {
    flag: Flag,
    window: Window,
}

impl ClientState {
    pub fn new(flag: Flag, window: Window) -> Self {
        Self { flag, window }
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn set_window(&mut self, window: Window) {
        self.window = window;
    }
}
