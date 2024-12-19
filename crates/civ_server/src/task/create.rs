use common::{network::message::CreateTaskMessage, task::CreateTaskError};
use uuid::Uuid;

use crate::{game::task::settle::Settle, runner::Runner};

use super::Task;

impl Runner {
    pub(crate) fn create_task(
        &self,
        task_id: Uuid,
        message: CreateTaskMessage,
    ) -> Result<Box<dyn Task + Send>, CreateTaskError> {
        match message {
            CreateTaskMessage::Settle(unit_uuid, city_name) => {
                //
                Ok(Box::new(Settle::new(
                    task_id,
                    self.context.context.clone(),
                    self.state(),
                    &unit_uuid,
                    city_name.clone(),
                )?))
            }
        }
    }
}
