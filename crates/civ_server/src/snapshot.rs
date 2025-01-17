use std::{collections::HashMap, fs, io, path::PathBuf};

use common::{
    game::{GameFrame, PlayerId},
    network::ClientId,
    space::window::Window,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::{city::City, unit::Unit},
    state::{
        clients::{ClientState, Clients},
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
    client_windows: Vec<(ClientId, Window)>,
    client_states: HashMap<PlayerId, ClientState>,
}

#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("Serialize/Deserialize error: {0}")]
    Serialize(#[from] bincode::Error),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

impl Snapshot {
    pub fn dump(&self, path: &PathBuf) -> Result<(), SnapshotError> {
        fs::write(path, bincode::serialize(&self)?)?;
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

    pub fn client_windows(&self) -> &[(ClientId, Window)] {
        &self.client_windows
    }

    pub fn client_states(&self) -> &HashMap<PlayerId, ClientState> {
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
            client_windows: value.clients().client_windows().to_vec(),
            client_states: value.clients().states().clone(),
        }
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
            Clients::new(value.client_windows, value.client_states),
            index,
            tasks,
            value.cities,
            value.units,
            0,
        )
    }
}
