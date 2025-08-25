use std::{collections::HashMap, fs, io, path::PathBuf};

use common::{
    game::{GameFrame, PlayerId},
    space::D2Size,
    utils::Vec2d,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::{city::City, unit::Unit},
    runner::RunnerContext,
    state::{
        clients::{Clients, PlayerState},
        index::Index,
        State,
    },
    task::{Task, TaskBox},
};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    frame_i: GameFrame,
    world_size: D2Size,
    pub tasks: Vec<Box<dyn Task>>,
    cities: Vec2d<Box<City>>,
    cities_count: usize,
    units: Vec2d<Vec<Unit>>,
    units_count: usize,
    client_states: HashMap<PlayerId, PlayerState>,
}

#[derive(Debug, Error, Clone)]
pub enum SnapshotError {
    #[error("Serialize/Deserialize error: {0}")]
    Serialize(String),
    #[error("I/O error: {0}")]
    Io(io::ErrorKind),
}

impl Snapshot {
    pub fn dump(&self, path: &PathBuf) -> Result<(), SnapshotError> {
        fs::write(
            path,
            bincode::serialize(&self).map_err(|e| SnapshotError::Serialize(e.to_string()))?,
        )
        .map_err(|e| SnapshotError::Io(e.kind()))?;
        Ok(())
    }

    pub fn frame_i(&self) -> GameFrame {
        self.frame_i
    }

    pub fn tasks(&self) -> &[Box<dyn Task>] {
        &self.tasks
    }

    pub fn cities(&self) -> &Vec2d<Box<City>> {
        &self.cities
    }

    pub fn units(&self) -> &Vec2d<Vec<Unit>> {
        &self.units
    }

    pub fn client_states(&self) -> &HashMap<PlayerId, PlayerState> {
        &self.client_states
    }
}

impl From<&RunnerContext> for Snapshot {
    fn from(value: &RunnerContext) -> Self {
        let tasks = value
            .tasks
            .read()
            .into_iter()
            .map(|t| t.clone())
            .map(|t| t as _)
            .collect();
        let state = value.state.read().unwrap();
        Self {
            frame_i: *state.frame(),
            world_size: state.world_size(),
            tasks,
            cities: state.cities().clone(),
            cities_count: state.cities_count(),
            units: state.units().clone(),
            units_count: state.units_count(),
            client_states: state.clients().states().clone(),
        }
    }
}

impl TryFrom<&PathBuf> for Snapshot {
    type Error = SnapshotError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        bincode::deserialize(&fs::read(value).map_err(|e| SnapshotError::Io(e.kind()))?)
            .map_err(|e| SnapshotError::Serialize(e.to_string()))
    }
}

impl From<Snapshot> for State {
    fn from(value: Snapshot) -> Self {
        let index = Index::from(&value);
        Self::new(
            value.frame_i,
            Clients::new(value.client_states),
            vec![],
            index,
            value.cities,
            value.cities_count,
            value.units,
            value.units_count,
            value.world_size,
            0,
        )
    }
}
