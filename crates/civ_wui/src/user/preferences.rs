use std::any::type_name;
use std::{collections::HashMap, fs, io};

use bevy::prelude::{Deref, DerefMut};
use common::game::PlayerId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::network::ServerAddress;
use crate::utils::app_dir;

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize)]
struct PlayerIds(HashMap<ServerAddress, PlayerId>);

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize)]
struct KeepConnected(HashMap<ServerAddress, bool>);

#[derive(Debug)]
pub struct Preferences {
    player_id: PlayerIds,
    keep_connected: KeepConnected,
}

impl Preferences {
    pub fn from_env() -> Result<Self, PreferencesError> {
        let player_id = read::<PlayerIds>()?;
        let keep_connected = read::<KeepConnected>()?;

        Ok(Self {
            player_id,
            keep_connected,
        })
    }

    pub fn player_id(&self, server: &ServerAddress) -> Option<&PlayerId> {
        self.player_id.get(server)
    }

    pub fn set_player_id(&mut self, server: &ServerAddress, value: &PlayerId) {
        self.player_id.insert(server.clone(), *value);
    }

    pub fn keep_connected(&self, server: &ServerAddress) -> Option<&bool> {
        self.keep_connected.get(server)
    }

    pub fn set_keep_connected(&mut self, server: &ServerAddress, value: bool) {
        self.keep_connected.insert(server.clone(), value);
    }
}

#[derive(Debug, Error)]
pub enum PreferencesError {
    #[error("Io error: {0}")]
    Io(io::ErrorKind),
    #[error("Deserialize error: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("Can't determine home")]
    CantDetermineHome,
}

#[cfg(not(target_arch = "wasm32"))]
fn read<T: for<'a> Deserialize<'a>>() -> Result<T, PreferencesError> {
    let file_path = app_dir()
        .ok_or(PreferencesError::CantDetermineHome)?
        .join(format!("{}.json", type_name::<T>()));
    Ok(serde_json::from_str(
        &fs::read_to_string(file_path).map_err(|e| PreferencesError::Io(e.kind()))?,
    )?)
}
