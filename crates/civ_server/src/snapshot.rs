use std::{collections::HashMap, fs, io, path::PathBuf};

use common::game::{GameFrame, PlayerId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::{city::City, unit::Unit},
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
    tasks: Vec<Box<dyn Task>>,
    cities: Vec<City>,
    units: Vec<Unit>,
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

    pub fn cities(&self) -> &[City] {
        &self.cities
    }

    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn client_states(&self) -> &HashMap<PlayerId, PlayerState> {
        &self.client_states
    }
}

impl From<&State> for Snapshot {
    fn from(value: &State) -> Self {
        let tasks = value
            .tasks()
            .clone()
            .into_iter()
            .map(|bx| bx as _)
            .collect();
        Self {
            frame_i: *value.frame(),
            tasks,
            cities: value.cities().to_vec(),
            units: value.units().to_vec(),
            client_states: value.clients().states().clone(),
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
        let tasks: Vec<TaskBox> = value
            .tasks
            .clone()
            .into_iter()
            .map(|bx| bx.boxed())
            .collect();
        Self::new(
            value.frame_i,
            Clients::new(value.client_states),
            index,
            tasks,
            value.cities,
            value.units,
            0,
        )
    }
}
