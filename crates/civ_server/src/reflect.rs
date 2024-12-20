use std::sync::MutexGuard;

use common::{
    geo::{Geo, GeoContext, WorldPoint},
    network::message::{ClientStateMessage, ServerToClientMessage},
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
    state::{State, StateError},
    task::{
        effect::{CityEffect, Effect, StateEffect, TaskEffect, UnitEffect},
        Concern, IntoClientTask, TaskBox,
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
        let state = &self.state();
        match effect {
            Effect::State(effect) => match effect {
                StateEffect::Client(_, _) => Ok(None),
                StateEffect::Task(_, effect) => match effect {
                    TaskEffect::Push(task) => self.new_task_reflects(task, state),
                    TaskEffect::Finished(uuid) => self.finished_task_reflects(uuid, state),
                },
                StateEffect::City(_, effect) => match effect {
                    CityEffect::New(city) => self.new_city_reflects(city, state),
                    CityEffect::Remove(uuid) => self.removed_city_reflects(uuid, state),
                },
                StateEffect::Unit(_, effect) => match effect {
                    UnitEffect::New(unit) => self.new_unit_reflects(unit, state),
                    UnitEffect::Remove(uuid) => self.removed_unit_reflects(uuid, state),
                    UnitEffect::Move(uuid, to_) => self.moved_unit_reflects(uuid, to_, state),
                },
            },
        }
    }

    fn task_point(
        &self,
        task: &TaskBox,
        // TODO: State into RwLock instead
        state: &MutexGuard<State>,
    ) -> Result<Option<GeoContext>, ReflectError> {
        match task.concern() {
            Concern::Nothing => Ok(None),
            Concern::Unit(uuid) => Ok(Some(*state.find_unit(&uuid)?.geo())),
            Concern::City(uuid) => Ok(Some(*state.find_city(&uuid)?.geo())),
        }
    }

    fn new_task_reflects(
        &self,
        task: &TaskBox,
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        if let Some(geo) = self.task_point(task, state)? {
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
        task_id: &Uuid,
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        // FIXME: not good, hopefully state is modified after ... Transport task in TaskEffect::Finished
        let task = state
            .tasks()
            .iter()
            .find(|t| t.context().id() == *task_id)
            .unwrap();

        if let Some(geo) = self.task_point(task, state)? {
            let clients = state.clients().concerned(&geo);
            if !clients.is_empty() {
                match task.concern() {
                    Concern::Unit(uuid) => {
                        return Ok(Some((
                            ServerToClientMessage::State(ClientStateMessage::RemoveUnitTask(
                                uuid, *task_id,
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
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let clients = state.clients().concerned(city.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::AddCity(city.into_client())),
                clients,
            )));
        }

        Ok(None)
    }

    fn removed_city_reflects(
        &self,
        city_id: &Uuid,
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        if let Ok(city) = state.find_city(city_id) {
            let clients = state.clients().concerned(city.geo());
            if !clients.is_empty() {
                return Ok(Some((
                    ServerToClientMessage::State(ClientStateMessage::RemoveCity(*city_id)),
                    clients,
                )));
            }
        }

        Ok(None)
    }

    fn new_unit_reflects(
        &self,
        unit: &Unit,
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        let clients = state.clients().concerned(&unit.geo());
        if !clients.is_empty() {
            return Ok(Some((
                ServerToClientMessage::State(ClientStateMessage::AddUnit(unit.into_client(state))),
                clients,
            )));
        }

        Ok(None)
    }

    fn removed_unit_reflects(
        &self,
        unit_id: &Uuid,
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        if let Some(unit) = state.find_unit(unit_id).ok() {
            let clients = state.clients().concerned(&unit.geo());
            if !clients.is_empty() {
                return Ok(Some((
                    ServerToClientMessage::State(ClientStateMessage::RemoveUnit(*unit_id)),
                    clients,
                )));
            }
        }

        Ok(None)
    }

    fn moved_unit_reflects(
        &self,
        unit_id: &Uuid,
        to_: &WorldPoint,
        state: &MutexGuard<State>,
    ) -> Result<Option<(ServerToClientMessage, Vec<Uuid>)>, ReflectError> {
        if let Some(unit) = state.find_unit(unit_id).ok() {
            let clients = state.clients().concerned(unit.geo());
            if !clients.is_empty() {
                return Ok(Some((
                    ServerToClientMessage::State(ClientStateMessage::MoveUnit(*unit_id, *to_)),
                    clients,
                )));
            }
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
