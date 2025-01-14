use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod message;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client(Uuid, Uuid); // ClientId, PlayerId

impl Client {
    pub fn new(client_id: Uuid, player_id: Uuid) -> Self {
        Self(client_id, player_id)
    }

    pub fn client_id(&self) -> &Uuid {
        &self.0
    }

    pub fn player_id(&self) -> &Uuid {
        &self.1
    }
}
