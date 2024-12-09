use message_io::network::Endpoint;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct Clients {
    endpoints: HashMap<Uuid, Endpoint>,
    clients: HashMap<Endpoint, Uuid>,
}

impl Clients {
    pub fn insert(&mut self, client_id: Uuid, endpoint: Endpoint) {
        self.endpoints.insert(client_id, endpoint);
        self.clients.insert(endpoint, client_id);
    }

    pub fn remove(&mut self, endpoint: &Endpoint) {
        if let Some(client_id) = self.clients.remove(endpoint) {
            self.endpoints.remove(&client_id);
        }
    }

    pub fn client_id(&self, endpoint: &Endpoint) -> Option<&Uuid> {
        self.clients.get(endpoint)
    }

    pub fn endpoint(&self, client_id: &Uuid) -> Option<&Endpoint> {
        self.endpoints.get(client_id)
    }

    pub fn length(&self) -> usize {
        self.clients.len()
    }
}
