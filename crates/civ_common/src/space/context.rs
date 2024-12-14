use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientPhysicalContext {
    x: u64,
    y: u64,
}

impl ClientPhysicalContext {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}
