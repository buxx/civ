use uuid::Uuid;

use super::CommandContext;

pub fn cities(context: CommandContext) {
    let state = context
        .state
        .lock()
        .expect("Consider state always accessible");

    for city in state.cities() {
        println!("{}: {}", city.id(), city.name())
    }
}

pub fn city(context: CommandContext, id: Uuid) {
    let state = context
        .state
        .lock()
        .expect("Consider state always accessible");

    if let Some(city) = state.cities().iter().find(|c| c.id() == id) {
        println!("id: {}", city.id());
        println!("name: {}", city.name());
        println!("xy: {:?}", city.geo().xy());
    }
}
