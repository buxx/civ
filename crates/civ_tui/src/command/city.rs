use uuid::Uuid;

use super::CommandContext;

pub fn cities(context: CommandContext) {
    let state = context
        .state
        .lock()
        .expect("Consider state always accessible");

    if let Some(cities) = state.cities() {
        for city in cities {
            println!("{}: {}", city.id(), city.name())
        }
    } else {
        println!("Game state not ready")
    }
}

pub fn city(context: CommandContext, id: Uuid) {
    let state = context
        .state
        .lock()
        .expect("Consider state always accessible");

    if let Some(cities) = state.cities() {
        if let Some(city) = cities.iter().find(|c| c.id() == id) {
            println!("id: {}", city.id());
            println!("name: {}", city.name());
            println!("xy: {:?}", city.geo().point());
        }
    } else {
        println!("Game state not ready")
    }
}
