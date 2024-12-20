use std::collections::HashMap;

use common::{geo::GeoContext, space::window::Window};
use uuid::Uuid;

use crate::task::effect::ClientEffect;

#[derive(Default)]
pub struct Clients {
    count: usize,
    states: HashMap<Uuid, ClientState>,
}

impl Clients {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn set_count(&mut self, value: usize) {
        self.count = value;
    }

    pub fn apply(&mut self, client_id: Uuid, effect: ClientEffect) {
        match effect {
            ClientEffect::SetWindow(window) => {
                self.states
                    .entry(client_id)
                    .or_default()
                    .set_window(Some(window));
            }
        }
    }

    pub fn concerned(&self, geo: &GeoContext) -> Vec<Uuid> {
        self.states
            .iter()
            .filter(|(_, state)| {
                state
                    .window
                    .as_ref()
                    .and_then(|w| Some(w.contains(geo)))
                    .unwrap_or(false)
            })
            .map(|(uuid, _)| *uuid)
            .collect()
    }

    pub fn client_ids(&self) -> Vec<Uuid> {
        self.states.keys().copied().collect()
    }
}

#[derive(Default)]
pub struct ClientState {
    window: Option<Window>,
}

impl ClientState {
    pub fn set_window(&mut self, window: Option<Window>) {
        self.window = window;
    }
}
