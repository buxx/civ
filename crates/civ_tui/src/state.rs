use common::{
    game::slice::{ClientCity, ClientUnit},
    network::message::ClientStateMessage,
    space::window::Window,
};
use uuid::Uuid;

use crate::error::PublicError;

pub struct State {
    client_id: Uuid,
    connected: bool,
    window: Option<Window>,
    errors: Vec<PublicError>,
    cities: Vec<ClientCity>,
    units: Vec<ClientUnit>,
}

impl State {
    pub fn new(client_id: Uuid) -> Self {
        Self {
            client_id,
            connected: false,
            window: None,
            errors: vec![],
            cities: vec![],
            units: vec![],
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

    pub fn apply(&mut self, message: ClientStateMessage) {
        match message {
            ClientStateMessage::SetWindow(window) => {
                self.set_window(Some(window));
            }
            ClientStateMessage::SetGameSlice(slice) => {
                self.cities = slice.cities().into();
                self.units = slice.units().into();
            }
            ClientStateMessage::AddCity(city) => self.cities.push(city),
            ClientStateMessage::RemoveCity(uuid) => self.cities.retain(|c| c.id() != uuid),
            ClientStateMessage::AddUnit(unit) => self.units.push(unit),
            ClientStateMessage::MoveUnit(uuid, to_) => {
                if let Some(u) = self.units.iter_mut().find(|u| u.id() == uuid) {
                    u.physics_mut().set_xy(to_)
                }
            }
            ClientStateMessage::RemoveUnit(uuid) => self.units.retain(|u| u.id() != uuid),
        }
    }

    pub fn cities(&self) -> &[ClientCity] {
        &self.cities
    }

    pub fn units(&self) -> &[ClientUnit] {
        &self.units
    }
}
