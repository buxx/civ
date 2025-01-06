use common::{
    geo::{Geo, GeoContext},
    network::message::{ClientStateMessage, ServerToClientMessage},
};
use log::error;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{city::City, unit::Unit, IntoClientModel},
    runner::Runner,
    state::StateError,
    task::{
        effect::{CityEffect, Effect, StateEffect, UnitEffect},
        Concern, TaskBox,
    },
};

impl Runner {
    pub(crate) fn reflects(&self, effects: &Vec<Effect>) {
        for effect in effects {
            match self.reflect(effect) {
                Ok(Some((message, client_ids))) => {
                    for client_id in client_ids {
                        self.context
                            .to_client_sender
                            .send((client_id, message.clone()))
                            .unwrap();
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    error!("Error during reflect effect '{:?}': {}", effect, e)
                }
            };
        }
    }

    fn reflect(
        &self,
        effect: &Effect,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        match effect {
            Effect::State(effect) => match effect {
                StateEffect::Client(_, _) => Ok(None),
                StateEffect::Task(_, _) => {
                    // Task are reflected into City & Unit in server side,
                    // then City & Units are entirely send to client
                    Ok(None)
                }
                StateEffect::Tasks(_) => {
                    // Task are reflected into City & Unit in server side,
                    // then City & Units are entirely send to client
                    Ok(None)
                }
                StateEffect::City(_, effect) => match effect {
                    CityEffect::New(city) => self.set_city_reflects(city),
                    CityEffect::Replace(city) => self.set_city_reflects(city),
                    CityEffect::Remove(city) => self.removed_city_reflects(city),
                },
                StateEffect::Unit(_, effect) => match effect {
                    UnitEffect::New(unit) => self.set_unit_reflects(unit),
                    UnitEffect::Remove(unit) => self.removed_unit_reflects(unit),
                    UnitEffect::Move(unit, _) => self.set_unit_reflects(unit),
                },
            },
        }
    }

    fn task_point(&self, task: &TaskBox) -> Result<Option<GeoContext>, ReflectError> {
        let state = self.state();

        match task.concern() {
            Concern::Unit(uuid) => Ok(Some(*state.find_unit(&uuid)?.geo())),
            Concern::City(uuid) => Ok(Some(*state.find_city(&uuid)?.geo())),
        }
    }

    fn set_city_reflects(
        &self,
        city: &City,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(city.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::SetCity(
                    city.clone().into_client(&state),
                )),
                clients,
            )));
        }

        Ok(None)
    }

    fn removed_city_reflects(
        &self,
        city: &City,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(city.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::RemoveCity(*city.id())),
                clients,
            )));
        }

        Ok(None)
    }

    fn set_unit_reflects(
        &self,
        unit: &Unit,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(unit.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::SetUnit(
                    unit.clone().into_client(&state),
                )),
                clients,
            )));
        }

        Ok(None)
    }

    fn removed_unit_reflects(
        &self,
        unit: &Unit,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(unit.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::RemoveUnit(unit.id())),
                clients,
            )));
        }

        Ok(None)
    }
}

#[derive(Error, Debug)]
pub enum ReflectError {
    #[error("Unexpected state: {0}")]
    UnexpectedState(UnexpectedStateError),
}

#[derive(Error, Debug)]
pub enum UnexpectedStateError {
    #[error("Missing city {0}")]
    MissingCity(Uuid),
    #[error("Missing unit {0}")]
    MissingUnit(Uuid),
}

impl From<StateError> for ReflectError {
    fn from(value: StateError) -> Self {
        match value {
            StateError::CityNotFound(_, uuid) => {
                ReflectError::UnexpectedState(UnexpectedStateError::MissingCity(uuid))
            }
            StateError::CityUuidFound(uuid) => {
                ReflectError::UnexpectedState(UnexpectedStateError::MissingCity(uuid))
            }
            StateError::UnitNotFound(_, uuid) => {
                ReflectError::UnexpectedState(UnexpectedStateError::MissingUnit(uuid))
            }
            StateError::UnitUuidNotFound(uuid) => {
                ReflectError::UnexpectedState(UnexpectedStateError::MissingUnit(uuid))
            }
        }
    }
}
