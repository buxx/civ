use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::game::{city::production::task::CityProductionTask, slice::ClientCityTasks};

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct CityTasks {
    pub production: CityProductionTask,
}

impl CityTasks {
    pub fn new(production: CityProductionTask) -> Self {
        Self { production }
    }
}

impl From<CityTasks> for ClientCityTasks {
    fn from(value: CityTasks) -> Self {
        ClientCityTasks::new(value.production.into())
    }
}
