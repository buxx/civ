use common::{
    geo::{Geo, GeoContext},
    network::message::{ClientStateMessage, ServerToClientMessage},
    world::reader::WorldReader,
};
use log::error;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{
        city::{City, IntoClientCity},
        unit::{IntoClientUnit, Unit},
    },
    runner::Runner,
    state::StateError,
    task::{
        effect::{CityEffect, Effect, StateEffect, TaskEffect, UnitEffect},
        Concern, IntoClientTask, TaskBox,
    },
};

impl<W: WorldReader + Sync + Send> Runner<W> {
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
                StateEffect::Task(_, effect) => match effect {
                    TaskEffect::Push(task) => self.new_task_reflects(task),
                    TaskEffect::Finished(task) => self.finished_task_reflects(task),
                },
                StateEffect::City(_, effect) => match effect {
                    CityEffect::New(city) => self.new_city_reflects(city),
                    CityEffect::Remove(city) => self.removed_city_reflects(city),
                },
                StateEffect::Unit(_, effect) => match effect {
                    UnitEffect::New(unit) => self.new_unit_reflects(unit),
                    UnitEffect::Remove(unit) => self.removed_unit_reflects(unit),
                    UnitEffect::Move(unit, _) => self.updated_unit_reflects(unit),
                },
            },
        }
    }

    fn task_point(&self, task: &TaskBox) -> Result<Option<GeoContext>, ReflectError> {
        let state = self.state();

        match task.concern() {
            Concern::Nothing => Ok(None),
            Concern::Unit(uuid) => Ok(Some(*state.find_unit(&uuid)?.geo())),
            Concern::City(uuid) => Ok(Some(*state.find_city(&uuid)?.geo())),
        }
    }

    fn new_task_reflects(
        &self,
        task: &TaskBox,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();

        if let Some(geo) = self.task_point(task)? {
            let clients = state.clients().concerned(&geo);
            if !clients.is_empty() {
                match task.concern() {
                    Concern::Unit(uuid) => {
                        return Ok(Some((
                            ServerToClientMessage::State(ClientStateMessage::AddUnitTask(
                                uuid,
                                task.into_client(),
                            )),
                            clients,
                        )));
                    }
                    Concern::City(_uuid) => todo!(),
                    Concern::Nothing => {}
                }
            }
        }

        Ok(None)
    }

    fn finished_task_reflects(
        &self,
        task: &TaskBox,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();

        if let Some(geo) = self.task_point(task)? {
            let clients = state.clients().concerned(&geo);
            if !clients.is_empty() {
                match task.concern() {
                    Concern::Unit(uuid) => {
                        return Ok(Some((
                            ServerToClientMessage::State(ClientStateMessage::RemoveUnitTask(
                                uuid,
                                task.context().id(),
                            )),
                            clients,
                        )));
                    }
                    Concern::City(_uuid) => todo!(),
                    Concern::Nothing => {}
                }
            }
        }

        Ok(None)
    }

    fn new_city_reflects(
        &self,
        city: &City,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(city.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::SetCity(city.into_client())),
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
                ServerToClientMessage::State(ClientStateMessage::RemoveCity(city.id())),
                clients,
            )));
        }

        Ok(None)
    }

    fn new_unit_reflects(
        &self,
        unit: &Unit,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(unit.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::SetUnit(unit.into_client(&state))),
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

    fn updated_unit_reflects(
        &self,
        unit: &Unit,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let state = self.state();
        let clients = state.clients().concerned(unit.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::SetUnit(unit.into_client(&state))),
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
