use common::game::GameFrame;
use std::path::PathBuf;

use crate::Args;

#[derive(Clone)]
pub struct ServerConfig {
    snapshot: Option<PathBuf>,
    snapshot_interval: GameFrame,
    tcp_listen_address: String,
    ws_listen_address: String,
}

impl ServerConfig {
    pub fn new(
        snapshot: Option<PathBuf>,
        snapshot_interval: GameFrame,
        tcp_listen_address: String,
        ws_listen_address: String,
    ) -> Self {
        Self {
            snapshot,
            snapshot_interval,
            tcp_listen_address,
            ws_listen_address,
        }
    }

    pub fn snapshot(&self) -> Option<&PathBuf> {
        self.snapshot.as_ref()
    }

    pub fn snapshot_interval(&self) -> &GameFrame {
        &self.snapshot_interval
    }

    pub fn tcp_listen_address(&self) -> &str {
        &self.tcp_listen_address
    }

    pub fn ws_listen_address(&self) -> &str {
        &self.ws_listen_address
    }
}

impl From<Args> for ServerConfig {
    fn from(value: Args) -> Self {
        Self {
            snapshot: value.snapshot,
            snapshot_interval: GameFrame(value.snapshot_interval),
            tcp_listen_address: value.tcp_listen_address,
            ws_listen_address: value.ws_listen_address,
        }
    }
}
