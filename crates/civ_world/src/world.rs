use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Builder)]
pub struct World {
    pub chunk_size: usize,
    pub width: usize,
    pub height: usize,
}
