use std::collections::HashMap;

use common::{
    game::{nation::flag::Flag, PlayerId},
    geo::GeoContext,
    network::{Client, ClientId},
    space::window::Window,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::effect::{ClientEffect, ClientsEffect};

#[derive(Default)]
pub struct Clients {
    index: Index,
    states: HashMap<PlayerId, PlayerState>,
}

// FIXME BS NOW: is that in snapshot ?
#[derive(Default)]
struct Index {
    client_player: HashMap<ClientId, PlayerId>,
    player_client: HashMap<PlayerId, ClientId>,
}

impl Index {
    fn insert(&mut self, client_id: ClientId, player_id: PlayerId) {
        self.client_player.insert(client_id, player_id);
        self.player_client.insert(player_id, client_id);
    }
}

#[derive(Debug, Error)]
pub enum ClientsError {
    #[error("Unknown client {0}")]
    UnknownClient(ClientId),
    #[error("Unknown player {0}")]
    UnknownPlayer(PlayerId),
}

impl Clients {
    pub fn new(states: HashMap<PlayerId, PlayerState>) -> Self {
        Self {
            index: Index::default(),
            states,
        }
    }

    pub fn clients_count(&self) -> usize {
        self.index.client_player.len()
    }

    pub fn players_count(&self) -> usize {
        self.states.len()
    }

    pub fn apply(&mut self, effect: &ClientsEffect) -> Result<(), ClientsError> {
        match effect {
            ClientsEffect::Insert(client_id, player_id) => {
                self.index.insert(*client_id, *player_id);
            }
        };

        Ok(())
    }

    pub fn apply_client(
        &mut self,
        client: &Client,
        effect: &ClientEffect,
    ) -> Result<(), ClientsError> {
        match effect {
            ClientEffect::PlayerTookPlace(flag, window) => {
                let state = PlayerState::new(*flag, *window);
                self.states.insert(*client.player_id(), state);
                self.index.insert(*client.client_id(), *client.player_id());
            }
            ClientEffect::SetWindow(window) => {
                if let Some(state) = self.states.get_mut(client.player_id()) {
                    state.set_window(*window);
                };
            }
        };

        Ok(())
    }

    pub fn concerned(&self, geo: &GeoContext) -> Vec<ClientId> {
        self.states
            .iter()
            .filter(|(_, state)| state.window.contains(geo))
            .filter_map(|(player_id, _)| self.index.player_client.get(player_id).cloned())
            .collect()
    }

    pub fn player_client_ids(&self) -> Vec<ClientId> {
        // TODO: Can be reference ?
        self.index.player_client.values().copied().collect()
    }

    pub fn player_state(&self, player_id: &PlayerId) -> Option<&PlayerState> {
        self.states.get(player_id)
    }

    pub fn flags(&self) -> Vec<Flag> {
        self.states.values().map(|s| *s.flag()).collect()
    }

    pub fn states(&self) -> &HashMap<PlayerId, PlayerState> {
        &self.states
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    flag: Flag,
    window: Window,
}

impl PlayerState {
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
