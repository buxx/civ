use std::collections::HashMap;

use common::{game::nation::flag::Flag, geo::GeoContext, network::Client, space::window::Window};
use thiserror::Error;
use uuid::Uuid;

use crate::effect::ClientEffect;

// FIXME: replace all Uuid by super types
#[derive(Default)]
pub struct Clients {
    count: usize,
    client_windows: Vec<(Uuid, Window)>,
    // this must be restored after a backup
    states: HashMap<Uuid, ClientState>, // PlayerId, ClientState
}

#[derive(Debug, Error)]
pub enum ClientsError {
    #[error("Unknown client {0}")]
    UnknownClient(Uuid),
    #[error("Unknown player {0}")]
    UnknownPlayer(Uuid),
}

impl Clients {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn set_count(&mut self, value: usize) {
        self.count = value;
    }

    pub fn apply(&mut self, client: &Client, effect: &ClientEffect) -> Result<(), ClientsError> {
        match effect {
            ClientEffect::PlayerTookPlace(flag) => {
                self.states
                    .insert(*client.player_id(), ClientState::new(*flag));
            }
            ClientEffect::SetWindow(window) => {
                self.client_windows
                    .push((*client.client_id(), window.clone()));
                self.states
                    .get_mut(client.player_id())
                    .ok_or(ClientsError::UnknownPlayer(*client.player_id()))?
                    .set_window(Some(window.clone()));
            }
        };

        Ok(())
    }

    pub fn concerned(&self, geo: &GeoContext) -> Vec<Uuid> {
        self.client_windows
            .iter()
            .filter_map(|(client, window)| {
                if window.contains(geo) {
                    Some(*client)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn client_ids(&self) -> Vec<Uuid> {
        self.states.keys().copied().collect()
    }

    pub fn player_state(&self, player_id: &Uuid) -> Option<&ClientState> {
        self.states.get(player_id)
    }

    pub fn flags(&self) -> Vec<Flag> {
        self.states.values().map(|s| *s.flag()).collect()
    }
}

pub struct ClientState {
    flag: Flag,
    window: Option<Window>,
}

impl ClientState {
    pub fn new(flag: Flag) -> Self {
        Self { flag, window: None }
    }

    pub fn set_window(&mut self, window: Option<Window>) {
        self.window = window;
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }
}
