use bon::Builder;
use common::game::{
    city::{CityId, CityProductionTons},
    tasks::client::city::production::ClientCityProductionTask,
    unit::{CityTaskType, TaskType},
};
use serde::{Deserialize, Serialize};

use crate::{
    effect::Effect,
    runner::RunnerContext,
    task::{Concern, Task, TaskBox, TaskContext, TaskError, Then},
};
