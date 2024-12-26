use common::{
    game::unit::{TaskType, UnitTaskType},
    network::message::{ClientToServerMessage, CreateTaskMessage},
};
use uuid::Uuid;

use super::{CommandContext, CommandError};

pub fn units(context: CommandContext) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    for unit in state.units()? {
        println!("{}", unit.id())
    }

    Ok(())
}

pub fn detail(context: CommandContext, id: Uuid) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    if let Some(unit) = state.units()?.iter().find(|c| c.id() == id) {
        let frame = state.frame()?;
        println!("id: {}", unit.id());
        println!("xy: {:?}", unit.geo().point());
        println!("type: {:?}", unit.type_().to_string());
        println!("tasks: {}", unit.tasks().display(&frame));
    }

    Ok(())
}

pub fn settle(context: CommandContext, unit_id: Uuid, city_name: &str) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Assume state always accessible");

    let unit = state
        .units()?
        .iter()
        .find(|c| c.id() == unit_id)
        .ok_or(CommandError::UnitNoMoreAvailable)?;
    if !context
        .context
        .rule_set()
        .unit_can(unit.type_())
        .contains(&TaskType::Unit(UnitTaskType::Settle))
    {
        println!("Action not available for this unit type");
        return Ok(());
    }

    context
        .to_server_sender
        .send(ClientToServerMessage::CreateTask(
            unit.id(),
            CreateTaskMessage::Settle(unit_id, city_name.to_string()),
        ))?;

    Ok(())
}
