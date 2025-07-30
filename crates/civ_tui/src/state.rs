use common::{
    game::{
        nation::flag::Flag,
        server::ServerResume,
        slice::{ClientCity, ClientUnit},
        GameFrame,
    },
    network::{message::ClientStateMessage, ClientId},
    space::window::Window,
    world::{partial::Slice, CtxTile, Tile},
};
use thiserror::Error;

use crate::error::PublicError;

pub struct State {
    client_id: ClientId,
    connected: bool,
    server: Option<ServerResume>,
    flag: Option<Flag>,
    window: Option<Window>,
    errors: Vec<PublicError>,
    frame: Option<GameFrame>,
    tiles: Option<Slice<CtxTile<Tile>>>,
    cities: Option<Vec<ClientCity>>,
    units: Option<Vec<ClientUnit>>,
}

impl State {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            connected: false,
            server: None,
            flag: None,
            window: None,
            errors: vec![],
            frame: None,
            tiles: None,
            cities: None,
            units: None,
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
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
                todo!()
                // self.tiles = Some(slice.tiles().clone());
                // self.cities = Some(slice.cities().into());
                // self.units = Some(slice.units().into());
            }
            ClientStateMessage::SetCity(city) => {
                if let Some(cities) = &mut self.cities {
                    cities.push(city)
                }
            }
            ClientStateMessage::RemoveCity(city_id) => {
                if let Some(cities) = &mut self.cities {
                    cities.retain(|c| c.id() != &city_id)
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
            ClientStateMessage::RemoveUnit(unit_id) => {
                if let Some(units) = &mut self.units {
                    units.retain(|u| u.id() != &unit_id)
                }
            }
        }
    }

    pub fn cities(&self) -> Result<&Vec<ClientCity>, StateError> {
        self.cities.as_ref().ok_or(StateError::NotReady)
    }

    pub fn units(&self) -> Result<&Vec<ClientUnit>, StateError> {
        self.units.as_ref().ok_or(StateError::NotReady)
    }

    pub fn frame(&self) -> Result<GameFrame, StateError> {
        self.frame.ok_or(StateError::NotReady)
    }

    pub fn tiles(&self) -> Option<&Slice<CtxTile<Tile>>> {
        self.tiles.as_ref()
    }

    pub fn server(&self) -> Option<&ServerResume> {
        self.server.as_ref()
    }

    pub fn set_server(&mut self, server: Option<ServerResume>) {
        self.server = server;
    }

    pub fn flag(&self) -> Option<Flag> {
        self.flag
    }

    pub fn set_flag(&mut self, flag: Option<Flag>) {
        self.flag = flag;
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Game state not ready")]
    NotReady,
}
