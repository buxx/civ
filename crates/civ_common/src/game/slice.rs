use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSlice {
    cities: Vec<ClientCity>,
    units: Vec<ClientUnit>,
}

impl GameSlice {
    pub fn new(cities: Vec<ClientCity>, units: Vec<ClientUnit>) -> Self {
        Self { cities, units }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCity {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUnit {}
