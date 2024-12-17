use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientGeoContext {
    x: u64,
    y: u64,
}

impl ClientGeoContext {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn xy(&self) -> (u64, u64) {
        (self.x, self.y)
    }

    pub fn set_xy(&mut self, to: (u64, u64)) {
        self.x = to.0;
        self.y = to.1;
    }
}
