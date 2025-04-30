use std::any::type_name;
use std::{collections::HashMap, fs, io};

use bevy::prelude::{Deref, DerefMut};
use common::game::PlayerId;
use common::network::ServerAddress;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
use wasm_cookies::CookieOptions;

#[cfg(not(target_arch = "wasm32"))]
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
        write_(&self.player_id).unwrap();
    }

    pub fn keep_connected(&self, server: &ServerAddress) -> Option<&bool> {
        self.keep_connected.get(server)
    }

    pub fn set_keep_connected(&mut self, server: &ServerAddress, value: bool) {
        self.keep_connected.insert(server.clone(), value);
        write_(&self.keep_connected).unwrap();
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
    #[error("Cookies related error: {0}")]
    CookieRelatedError(String),
}

#[cfg(not(target_arch = "wasm32"))]
fn read<T: for<'a> Deserialize<'a>>() -> Result<T, PreferencesError> {
    let file_path = app_dir()
        .ok_or(PreferencesError::CantDetermineHome)?
        .join(format!(
            "{}.json",
            type_name::<T>().split("::").last().unwrap()
        ));
    let raw = match fs::read_to_string(file_path) {
        Ok(raw) => Ok(raw),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => Ok("{}".to_string()),
            _ => Err(PreferencesError::Io(error.kind())),
        },
    }?;
    Ok(serde_json::from_str(&raw)?)
}

#[cfg(not(target_arch = "wasm32"))]
fn write_<T: Serialize>(value: T) -> Result<(), PreferencesError> {
    // FIXME: refactor file_path
    let file_path = app_dir()
        .ok_or(PreferencesError::CantDetermineHome)?
        .join(format!(
            "{}.json",
            type_name::<T>().split("::").last().unwrap()
        ));
    fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    fs::write(file_path, serde_json::to_string(&value)?).unwrap();
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn read<T: for<'a> Deserialize<'a>>() -> Result<T, PreferencesError> {
    let key = type_name::<T>().split("::").last().unwrap();
    let raw = match wasm_cookies::get(key) {
        Some(Err(error)) => return Err(PreferencesError::CookieRelatedError(error.to_string())),
        Some(Ok(value)) => value,
        None => "{}".to_string(),
    };
    Ok(serde_json::from_str(&raw)?)
}

#[cfg(target_arch = "wasm32")]
fn write_<T: Serialize>(value: T) -> Result<(), PreferencesError> {
    let key = type_name::<T>().split("::").last().unwrap();
    let value = serde_json::to_string(&value)?;
    wasm_cookies::set(key, &value, &CookieOptions::default());
    Ok(())
}
