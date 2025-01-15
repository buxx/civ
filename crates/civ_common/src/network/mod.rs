use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::PlayerId;

pub mod message;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(pub Uuid);

impl Default for ClientId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client(ClientId, PlayerId); // ClientId, PlayerId

impl Client {
    pub fn new(client_id: ClientId, player_id: PlayerId) -> Self {
        Self(client_id, player_id)
    }

    pub fn client_id(&self) -> &ClientId {
        &self.0
    }

    pub fn player_id(&self) -> &PlayerId {
        &self.1
    }
}
