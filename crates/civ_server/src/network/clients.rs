use common::network::{Client, ClientId};
use message_io::network::Endpoint;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Clients {
    endpoints: HashMap<ClientId, Endpoint>,
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

    pub fn endpoint(&self, client_id: &ClientId) -> Option<&Endpoint> {
        self.endpoints.get(client_id)
    }

    pub fn length(&self) -> usize {
        self.clients.len()
    }
}
