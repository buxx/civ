use bevy::prelude::*;

use derive_more::Constructor;

use crate::menu::gui::{LocalState, ServerState};

#[derive(Resource, Deref, Constructor)]
pub struct ContextResource(pub Context);

#[derive(Debug, Clone, Constructor)]
pub struct Context;

pub enum EntryPoint {
    Root,
    Server,
}

#[cfg(target_arch = "wasm32")]
impl Context {
    pub fn entry_point(&self) -> EntryPoint {
        EntryPoint::Server
    }

    pub fn default_server_address(&self) -> &str {
        // FIXME From query or 127.0.0.1
        "127.0.0.1:9877"
    }

    pub fn protocol(&self) -> &str {
        ""
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Context {
    pub fn entry_point(&self) -> EntryPoint {
        EntryPoint::Root
    }

    pub fn default_server_address(&self) -> &str {
        "127.0.0.1:9876"
    }

    pub fn protocol(&self) -> &str {
        ""
    }
}

impl From<Context> for LocalState {
    fn from(_: Context) -> Self {
        LocalState::new()
    }
}

impl From<Context> for ServerState {
    fn from(value: Context) -> Self {
        ServerState::new(format!(
            "{}{}",
            value.protocol(),
            value.default_server_address()
        ))
    }
}
