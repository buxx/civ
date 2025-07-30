use common::game::city::task::CityTasks;

use crate::task::TaskBox;

pub mod production;
pub mod settle;

impl From<CityTasks> for Vec<TaskBox> {
    fn from(value: CityTasks) -> Self {
        vec![Box::new(value.production)]
    }
}
