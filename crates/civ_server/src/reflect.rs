use common::{
    geo::Geo,
    network::{
        message::{ClientStateMessage, ServerToClientInGameMessage, ServerToClientMessage},
        ClientId,
    },
};
use log::error;
use thiserror::Error;

use crate::{
    effect::{CityEffect, Effect, StateEffect, UnitEffect},
    game::{city::City, unit::Unit, IntoClientModel},
    runner::Runner,
    state::StateError,
};

impl Runner {
    pub(crate) fn reflects(&self, effects: &Vec<Effect>) {
        for effect in effects {
            match self.reflect(effect) {
                Ok(reflects) => {
                    for (message, client_ids) in reflects {
                        for client_id in client_ids {
                            let _ = self
                                .context
                                .to_client_sender
                                .send_blocking((client_id, message.clone()));
                        }
                    }
                }
                Err(e) => {
                    error!("Error during reflect effect '{:?}': {}", effect, e)
                }
            };
        }
    }

    fn reflect(
        &self,
        effect: &Effect,
    ) -> Result<Vec<(ServerToClientMessage, Vec<ClientId>)>, ReflectError> {
        match effect {
            Effect::Shines(reflects) => Ok(reflects.clone()),
            Effect::State(effect) => match effect {
                StateEffect::Testing => Ok(vec![]),
                StateEffect::Clients(_) => Ok(vec![]),
                StateEffect::Client(_, _) => Ok(vec![]),
                StateEffect::Task(_, _) => {
                    // Task are reflected into City & Unit in server side,
                    // then City & Units are entirely send to client
                    Ok(vec![])
                }
                StateEffect::Tasks(_) => {
                    // Task are reflected into City & Unit in server side,
                    // then City & Units are entirely send to client
                    Ok(vec![])
                }
                StateEffect::City(_, effect) => match effect {
                    CityEffect::New(city) => self.set_city_reflects(city),
                    CityEffect::Replace(city) => self.set_city_reflects(city),
                    CityEffect::Remove(city) => self.removed_city_reflects(city),
                },
                StateEffect::Unit(_, effect) => match effect {
                    UnitEffect::New(unit) => self.set_unit_reflects(unit),
                    UnitEffect::Replace(unit) => self.set_unit_reflects(unit),
                    UnitEffect::Remove(unit) => self.removed_unit_reflects(unit),
                },
                StateEffect::IncrementGameFrame => self.increment_game_frame_reflects(),
            },
        }
    }

    // FIXME BS NOW: keep in memory (in client state ?) which client are still connected
    // to send only to connected
    fn increment_game_frame_reflects(
        &self,
    ) -> Result<Vec<(ServerToClientMessage, Vec<ClientId>)>, ReflectError> {
        let client_ids = self.state().clients().player_client_ids();
        let frame = *self.state().frame();
        Ok(vec![(
            ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                ClientStateMessage::SetGameFrame(frame),
            )),
            client_ids,
        )])
    }

    fn set_city_reflects(
        &self,
        city: &City,
    ) -> Result<Vec<(ServerToClientMessage, Vec<ClientId>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(city.geo());
        if !clients.is_empty() {
            return Ok(vec![(
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::SetCity(city.clone().into_client(&state)),
                )),
                clients,
            )]);
        }

        Ok(vec![])
    }

    fn removed_city_reflects(
        &self,
        city: &City,
    ) -> Result<Vec<(ServerToClientMessage, Vec<ClientId>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(city.geo());
        if !clients.is_empty() {
            return Ok(vec![(
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::RemoveCity(*city.geo().point(), *city.id()),
                )),
                clients,
            )]);
        }

        Ok(vec![])
    }

    fn set_unit_reflects(
        &self,
        unit: &Unit,
    ) -> Result<Vec<(ServerToClientMessage, Vec<ClientId>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(unit.geo());
        if !clients.is_empty() {
            return Ok(vec![(
                ClientStateMessage::SetUnit(unit.clone().into_client(&state)).into(),
                clients,
            )]);
        }

        Ok(vec![])
    }

    fn removed_unit_reflects(
        &self,
        unit: &Unit,
    ) -> Result<Vec<(ServerToClientMessage, Vec<ClientId>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(unit.geo());
        if !clients.is_empty() {
            return Ok(vec![(
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::RemoveUnit(*unit.geo().point(), *unit.id()),
                )),
                clients,
            )]);
        }

        Ok(vec![])
    }
}

#[derive(Error, Debug)]
pub enum ReflectError {
    #[error("Unexpected state: {0}")]
    UnexpectedState(#[from] StateError),
}
