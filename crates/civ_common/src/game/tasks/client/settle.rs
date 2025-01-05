use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientSettle {
    city_name: String,
}

impl ClientSettle {
    pub fn new(city_name: String) -> Self {
        Self { city_name }
    }

    pub fn city_name(&self) -> &str {
        &self.city_name
    }
}

impl Display for ClientSettle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Settle")
    }
}
