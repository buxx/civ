use std::collections::HashMap;

use common::space::window::Window;
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

    pub fn clients_displaying(&self, point: &(u64, u64)) -> Vec<Uuid> {
        let mut clients = vec![];

        for (uuid, state) in self.states.iter() {
            if let Some(window) = &state.window {
                if window.contains(point) {
                    clients.push(*uuid)
                }
            }
        }

        clients
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
