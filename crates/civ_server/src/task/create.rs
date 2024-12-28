use common::{
    network::message::CreateTaskMessage, task::CreateTaskError, world::reader::WorldReader,
};
use uuid::Uuid;

use crate::{game::task::settle::Settle, runner::Runner};

use super::TaskBox;

impl<W: WorldReader + Sync + Send> Runner<W> {
    pub(crate) fn create_task(
        &self,
        task_id: Uuid,
        message: CreateTaskMessage,
    ) -> Result<TaskBox, CreateTaskError> {
        match message {
            CreateTaskMessage::Settle(unit_uuid, city_name) => {
                if let Ok(unit) = self.state().find_unit(&unit_uuid) {
                    return Ok(Box::new(Settle::new(
                        task_id,
                        self.context.context.clone(),
                        self.state(),
                        unit.clone(),
                        city_name.clone(),
                    )?));
                }

                Err(CreateTaskError::IncoherentContext(
                    "Unit is not longer available".to_string(),
                    None,
                ))
            }
        }
    }
}
