use common::{
    game::{
        slice::{ClientCity, ClientUnit},
        GameFrame,
    },
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
    frame: Option<GameFrame>,
    cities: Option<Vec<ClientCity>>,
    units: Option<Vec<ClientUnit>>,
}

impl State {
    pub fn new(client_id: Uuid) -> Self {
        Self {
            client_id,
            connected: false,
            window: None,
            errors: vec![],
            frame: None,
            cities: None,
            units: None,
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
            ClientStateMessage::SetGameFrame(frame) => {
                self.frame = Some(frame);
            }
            ClientStateMessage::SetWindow(window) => {
                self.set_window(Some(window));
            }
            ClientStateMessage::SetGameSlice(slice) => {
                self.cities = Some(slice.cities().into());
                self.units = Some(slice.units().into());
            }
            ClientStateMessage::SetCity(city) => {
                if let Some(cities) = &mut self.cities {
                    cities.push(city)
                }
            }
            ClientStateMessage::RemoveCity(uuid) => {
                if let Some(cities) = &mut self.cities {
                    cities.retain(|c| c.id() != uuid)
                }
            }
            ClientStateMessage::SetUnit(unit) => {
                if let Some(units) = &mut self.units {
                    if let Some(unit_) = units.iter_mut().find(|u| u.id() == unit.id()) {
                        *unit_ = unit
                    } else {
                        units.push(unit)
                    }
                }
            }
            ClientStateMessage::RemoveUnit(uuid) => {
                if let Some(units) = &mut self.units {
                    units.retain(|u| u.id() != uuid)
                }
            }
            ClientStateMessage::AddUnitTask(uuid, task) => {
                if let Some(units) = &mut self.units {
                    if let Some(unit) = units.iter_mut().find(|u| u.id() == uuid) {
                        unit.tasks_mut().push(task)
                    }
                }
            }
            ClientStateMessage::RemoveUnitTask(unit_uuid, task_uuid) => {
                if let Some(units) = &mut self.units {
                    if let Some(unit) = units.iter_mut().find(|u| u.id() == unit_uuid) {
                        unit.tasks_mut().remove(task_uuid)
                    }
                }
            }
        }
    }

    pub fn cities(&self) -> &Option<Vec<ClientCity>> {
        &self.cities
    }

    pub fn units(&self) -> &Option<Vec<ClientUnit>> {
        &self.units
    }

    pub fn frame(&self) -> Option<GameFrame> {
        self.frame
    }
}
