use bevy::prelude::*;

use common::network::ServerAddress;
use derive_more::Constructor;

#[derive(Resource, Deref, Constructor)]
pub struct ContextResource(pub Context);

#[derive(Debug, Clone, Constructor)]
pub struct Context;

pub enum EntryPoint {
    Root,
    #[allow(unused)]
    Join,
}

#[cfg(target_arch = "wasm32")]
impl Context {
    pub fn entry_point(&self) -> EntryPoint {
        EntryPoint::Join
    }

    pub fn default_server_address(&self) -> ServerAddress {
        // FIXME From query or 127.0.0.1
        ServerAddress("127.0.0.1:9877".to_string())
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

    pub fn default_server_address(&self) -> ServerAddress {
        ServerAddress("127.0.0.1:9876".to_string())
    }
}
