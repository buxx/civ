use common::network::message::{ClientToServerMessage, CreateTaskMessage};
use uuid::Uuid;

use super::CommandContext;

pub fn units(context: CommandContext) {
    let state = context
        .state
        .lock()
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
        .lock()
        .expect("Consider state always accessible");

    if let (Some(frame), Some(units)) = (state.frame(), state.units()) {
        if let Some(unit) = units.iter().find(|c| c.id() == id) {
            println!("id: {}", unit.id());
            println!("xy: {:?}", unit.geo().xy());
            println!("type: {:?}", unit.type_().to_string());
            println!("tasks: {}", unit.tasks().display(&frame));
        }
    } else {
        println!("Game state not ready")
    }
}

pub fn settle(context: CommandContext, unit_id: Uuid) {
    context
        .to_server_sender
        .send(ClientToServerMessage::CreateTask(
            Uuid::new_v4(),
            CreateTaskMessage::Settle(unit_id, "City name".to_string()),
        ))
        .unwrap()
}
