use common::{
    geo::Geo,
    network::message::{ClientStateMessage, ServerToClientMessage},
};
use uuid::Uuid;

use crate::{
    game::extractor::Extractor,
    runner::Runner,
    task::effect::{CityEffect, Effect, StateEffect, TaskEffect, UnitEffect},
};

impl Runner {
    pub(crate) fn reflects(&self, effects: &Vec<Effect>) {
        for effect in effects {
            if let Some((message, client_ids)) = self.reflect(effect) {
                for client_id in client_ids {
                    self.context
                        .to_client_sender
                        .send((client_id, message.clone()))
                        .unwrap()
                }
            }
        }
    }

    fn reflect(&self, effect: &Effect) -> Option<(ServerToClientMessage, Vec<Uuid>)> {
        let state = &self.state();
        match effect {
            Effect::State(effect) => match effect {
                StateEffect::Client(_, _) => return None,
                StateEffect::Task(_, effect) => {
                    match effect {
                        TaskEffect::Push(task) => {
                            // FIXME: how to be sure about unit_uuid ?
                            let unit_uuid = task.concerned_unit().unwrap();
                            if let Some(point) = task
                                .concerned_unit()
                                // TODO: should be an error if not Ok ?
                                .and_then(|unit_uuid| state.find_unit(&unit_uuid).ok())
                                .map(|u| u.geo().xy())
                            {
                                let task = Extractor::new(state).task_into_client(task);
                                let message = ServerToClientMessage::State(
                                    ClientStateMessage::AddUnitTask(unit_uuid, task),
                                );
                                let client_ids = state.clients().clients_displaying(&point);
                                return Some((message, client_ids));
                            }
                        }
                        TaskEffect::Finished(uuid) => {
                            // FIXME: not good, hopefully state is modified after ...
                            let task = state
                                .tasks()
                                .iter()
                                .find(|t| t.context().id() == *uuid)
                                .unwrap();
                            // TODO: another way to know client is concerned ?
                            // FIXME: use task index by uuid to avoid performance bottleneck here; REF PERF_TASK
                            if let Some(concerned_unit) = state
                                .tasks()
                                .iter()
                                .find(|t| t.context().id() == *uuid)
                                .and_then(|task| task.concerned_unit())
                            {
                                // TODO: should be an error if not Ok ?
                                if let Some(point) =
                                    state.find_unit(&concerned_unit).ok().map(|u| u.geo().xy())
                                {
                                    let message = ServerToClientMessage::State(
                                        ClientStateMessage::RemoveUnitTask(concerned_unit, *uuid),
                                    );
                                    let client_ids = state.clients().clients_displaying(&point);
                                    return Some((message, client_ids));
                                }
                            }
                        }
                    }
                }
                StateEffect::City(_, effect) => {
                    //
                    match effect {
                        CityEffect::New(city) => {
                            let city = Extractor::new(state).city_into_client(city);
                            let message = ServerToClientMessage::State(
                                ClientStateMessage::AddCity(city.clone()),
                            );
                            let client_ids = state.clients().clients_displaying(&city.geo().xy());
                            return Some((message, client_ids));
                        }
                        CityEffect::Remove(uuid) => {
                            let message =
                                ServerToClientMessage::State(ClientStateMessage::RemoveCity(*uuid));
                            if let Some(point) = state.find_city(uuid).ok().map(|c| c.geo().xy()) {
                                let client_ids = state.clients().clients_displaying(&point);
                                return Some((message, client_ids));
                            }
                        }
                    }
                }
                StateEffect::Unit(_, effect) => {
                    //
                    match effect {
                        UnitEffect::New(unit) => {
                            let unit = Extractor::new(state).unit_into_client(unit);
                            let message = ServerToClientMessage::State(
                                ClientStateMessage::AddUnit(unit.clone()),
                            );
                            let client_ids = state.clients().clients_displaying(&unit.geo().xy());
                            return Some((message, client_ids));
                        }
                        UnitEffect::Remove(uuid) => {
                            let message =
                                ServerToClientMessage::State(ClientStateMessage::RemoveUnit(*uuid));
                            if let Some(point) = state.find_unit(uuid).ok().map(|u| u.geo().xy()) {
                                let client_ids = state.clients().clients_displaying(&point);
                                return Some((message, client_ids));
                            }
                        }
                        UnitEffect::Move(uuid, to_) => {
                            let message = ServerToClientMessage::State(
                                ClientStateMessage::MoveUnit(*uuid, *to_),
                            );
                            let client_ids = state.clients().clients_displaying(&to_);
                            return Some((message, client_ids));
                        }
                    }
                }
            },
        }

        None
    }
}
