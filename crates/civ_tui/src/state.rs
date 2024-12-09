use uuid::Uuid;

pub struct State {
    client_id: Uuid,
}

impl State {
    pub fn new(client_id: Uuid) -> Self {
        Self { client_id }
    }

    pub fn client_id(&self) -> Uuid {
        self.client_id
    }
}
