use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Window {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl Window {
    pub fn new(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}:{} {}:{}",
            self.start_x, self.start_y, self.end_x, self.end_y,
        ))
    }
}
