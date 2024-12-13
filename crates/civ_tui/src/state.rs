use common::space::window::Window;
use uuid::Uuid;

use crate::error::PublicError;

pub struct State {
    client_id: Uuid,
    connected: bool,
    window: Option<Window>,
    errors: Vec<PublicError>,
}

impl State {
    pub fn new(client_id: Uuid) -> Self {
        Self {
            client_id,
            connected: false,
            window: None,
            errors: vec![],
        }
    }

    pub fn client_id(&self) -> Uuid {
        self.client_id
    }

    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
    }

    pub fn set_window(&mut self, window: Option<Window>) {
        self.window = window;
    }

    pub fn errors(&self) -> &[PublicError] {
        &self.errors
    }

    pub fn push_error(&mut self, error: PublicError) {
        self.errors.push(error);
    }

    pub fn clear_error(&mut self) {
        self.errors.clear();
    }
}
