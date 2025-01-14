use std::thread;

use common::{
    game::unit::{TaskType, UnitTaskType},
    network::message::{
        ClientToServerGameMessage, ClientToServerInGameMessage, ClientToServerMessage,
        ClientToServerUnitMessage,
    },
};
use uuid::Uuid;

use super::{CommandContext, CommandError, FOLLOW_INTERVAL};

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

pub fn detail(context: CommandContext, id: Uuid, follow: bool) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    let mut follow_ = true;
    while follow_ && !context.context.stop_is_required() {
        if let Some(unit) = state.units()?.iter().find(|c| c.id() == id) {
            let frame = state.frame()?;
            let task_str = unit
                .task()
                .as_ref()
                .map(|t| t.to_string(&frame))
                .unwrap_or("Idle".to_string());
            println!("id: {}", unit.id());
            println!("xy: {:?}", unit.geo().point());
            println!("type: {:?}", unit.type_().to_string());
            println!("task: {}", task_str);
        }
        follow_ = follow;

        if follow_ {
            thread::sleep(FOLLOW_INTERVAL);
        }
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

    context.to_server_sender.send(ClientToServerMessage::Game(
        ClientToServerGameMessage::InGame(ClientToServerInGameMessage::Unit(
            unit.id(),
            ClientToServerUnitMessage::Settle(city_name.to_string()),
        )),
    ))?;

    Ok(())
}
