use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Window {
    from_x: u32,
    from_y: u32,
    to_x: u32,
    to_y: u32,
}

impl Window {
    pub fn new(from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> Self {
        Self {
            from_x,
            from_y,
            to_x,
            to_y,
        }
    }
}
