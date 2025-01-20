use common::game::GameFrame;
use std::path::PathBuf;

use crate::Args;

#[derive(Clone)]
pub struct ServerConfig {
    snapshot: Option<PathBuf>,
    snapshot_interval: GameFrame,
}

impl ServerConfig {
    pub fn new(snapshot: Option<PathBuf>, snapshot_interval: GameFrame) -> Self {
        Self {
            snapshot,
            snapshot_interval,
        }
    }

    pub fn snapshot(&self) -> Option<&PathBuf> {
        self.snapshot.as_ref()
    }

    pub fn snapshot_interval(&self) -> &GameFrame {
        &self.snapshot_interval
    }
}

impl From<Args> for ServerConfig {
    fn from(value: Args) -> Self {
        Self {
            snapshot: value.snapshot,
            snapshot_interval: GameFrame(value.snapshot_interval),
        }
    }
}
