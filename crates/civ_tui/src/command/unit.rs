use common::{
    game::unit::{TaskType, UnitTaskType},
    network::message::{ClientToServerMessage, CreateTaskMessage},
};
use uuid::Uuid;

use super::CommandContext;

pub fn units(context: CommandContext) {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    if let Some(units) = state.units() {
        for unit in units {
            println!("{}", unit.id())
        }
    } else {
        println!("Game state not ready")
    }
}

pub fn detail(context: CommandContext, id: Uuid) {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    if let (Some(frame), Some(units)) = (state.frame(), state.units()) {
        if let Some(unit) = units.iter().find(|c| c.id() == id) {
            println!("id: {}", unit.id());
            println!("xy: {:?}", unit.geo().point());
            println!("type: {:?}", unit.type_().to_string());
            println!("tasks: {}", unit.tasks().display(&frame));
        }
    } else {
        println!("Game state not ready")
    }
}

pub fn settle(context: CommandContext, unit_id: Uuid, city_name: &str) {
    let state = context
        .state
        .read()
        .expect("Assume state always accessible");

    if let Some(units) = state.units() {
        if let Some(unit) = units.iter().find(|c| c.id() == unit_id) {
            if !context
                .context
                .rule_set()
                .unit_can(unit.type_())
                .contains(&TaskType::Unit(UnitTaskType::Settle))
            {
                println!("Action not available for this unit type");
                return;
            }

            context
                .to_server_sender
                .send(ClientToServerMessage::CreateTask(
                    unit.id(),
                    CreateTaskMessage::Settle(unit_id, city_name.to_string()),
                ))
                .unwrap()
        } else {
            println!("Unit no more available")
        }
    } else {
        println!("Game state not ready")
    }
}
