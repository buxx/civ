use uuid::Uuid;

use super::CommandContext;

pub fn units(context: CommandContext) {
    let state = context
        .state
        .lock()
        .expect("Consider state always accessible");
    for unit in state.units() {
        println!("{}", unit.id())
    }
}

pub fn detail(context: CommandContext, id: Uuid) {
    let state = context
        .state
        .lock()
        .expect("Consider state always accessible");

    if let Some(unit) = state.units().iter().find(|c| c.id() == id) {
        println!("id: {}", unit.id());
        println!("xy: {:?}", unit.physics().xy());
        println!("type: {:?}", unit.type_().to_string());
        println!("tasks: {}", unit.tasks());
    }
}
