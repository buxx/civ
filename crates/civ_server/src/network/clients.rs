use common::network::Client;
use message_io::network::Endpoint;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct Clients {
    endpoints: HashMap<Uuid, Endpoint>,
    clients: HashMap<Endpoint, Client>,
}

impl Clients {
    pub fn insert(&mut self, client: Client, endpoint: Endpoint) {
        self.endpoints.insert(*client.client_id(), endpoint);
        self.clients.insert(endpoint, client);
    }

    pub fn remove(&mut self, endpoint: &Endpoint) {
        if let Some(client) = self.clients.remove(endpoint) {
            self.endpoints.remove(client.client_id());
        }
    }

    pub fn client_for_endpoint(&self, endpoint: &Endpoint) -> Option<&Client> {
        self.clients.get(endpoint)
    }

    pub fn endpoint(&self, client_id: &Uuid) -> Option<&Endpoint> {
        self.endpoints.get(client_id)
    }

    pub fn length(&self) -> usize {
        self.clients.len()
    }
}
